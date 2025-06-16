import json
import random
from wasmtime import Store, Module, Instance, Linker, Func, Memory
import ctypes

class JsonEncoderWasm:
    def __init__(self, wasm_path: str):
        self.store = Store()
        self.module = Module.from_file(self.store.engine, wasm_path)
        self.linker = Linker(self.store.engine)
        self.instance = Instance(self.store, self.module, [])
        self.encode_func: Func = self.instance.exports(self.store)["encode_from_json"]
        self.malloc_func: Func = self.instance.exports(self.store)["malloc"]
        self.free_func: Func = self.instance.exports(self.store)["free_ptr"]
        self.memory: Memory = self.instance.exports(self.store)["memory"]

    def _write_string(self, s: str) -> int:
        data = s.encode("utf-8") + b"\0"
        size = len(data)
        ptr = self.malloc_func(self.store, size)

        buf_ptr = self.memory.data_ptr(self.store)

        dest = ctypes.c_void_p(ctypes.addressof(buf_ptr.contents) + ptr)
        ctypes.memmove(dest, data, size)
        return ptr

    def _read_cstring(self, ptr: int) -> str:
        buf_ptr = self.memory.data_ptr(self.store)
        c_ptr = ctypes.c_char_p(ctypes.addressof(buf_ptr.contents) + ptr)
        return c_ptr.value.decode("utf-8")

    def encode_from_json(self, obj: dict) -> str:
        json_str = json.dumps(obj, ensure_ascii=False)
        ptr = self._write_string(json_str)

        result_ptr = self.encode_func(self.store, ptr)
        if not result_ptr:
            raise RuntimeError("encode_from_json 返回空指针")

        hex_str = self._read_cstring(result_ptr)

        self.free_func(self.store, result_ptr)
        return hex_str

    @staticmethod
    def fill_random_fields(obj: dict) -> dict:
        obj["seq"] = random.randint(0, 0xFFFFFFFF)
        obj["random_number"] = random.randint(0, 0xFFFFFFFF)
        return obj


if __name__ == "__main__":
    encoder = JsonEncoderWasm("qq_msg_encoding.wasm")

    data = {
        "message_type": "group", # or user
        "peer_id": 123, # group_id or user_id
        "message": [
            {"type": "text", "data": {"text": "你好世界"}},
            {"type": "keyboard", "data": {"rows": [
                {"buttons": [
                    {
                        "id": "1",
                        "render_data": {"label": "⬅️上一页", "visited_label": "⬅️上一页", "style": 1},
                        "action": {
                            "type": 2,
                            "permission": {"type": 2, "specify_role_ids": [], "specify_user_ids": []},
                            "unsupport_tips": "兼容文本",
                            "data": "data",
                            "reply": True,
                            "enter": True
                        }
                    }
                ]}
            ]}}
        ],
        "seq": 12345678,
        "random_number": 123456789
    }

    data = encoder.fill_random_fields(data) # optional
    hex_result = encoder.encode_from_json(data)
    print("HEX:", hex_result)
