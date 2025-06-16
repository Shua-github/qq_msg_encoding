use maplit::hashmap;
use hex;
use serde::Deserialize;
use crate::protobuf::ProtobufEncoder;
use crate::protobuf::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct KeyboardData {
    pub rows: Vec<Row>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    pub buttons: Vec<Button>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Button {
    pub id: String,
    pub render_data: RenderData,
    pub action: Action,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: i32,
    pub permission: Permission,
    pub unsupport_tips: String,
    pub data: String,
    pub reply: bool,
    pub enter: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Permission {
    #[serde(rename = "type")]
    pub permission_type: i32,
    pub specify_role_ids: Vec<String>,
    pub specify_user_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RenderData {
    pub label: String,
    pub visited_label: String,
    pub style: i32,
}

#[derive(Debug, Clone)]
pub enum MessageElement {
    Text(String),
    #[allow(dead_code)]
    Keyboard(KeyboardData),
}

fn element_to_protobuf_value(element: &MessageElement) -> Value {
    match element {
        MessageElement::Text(text) => {
            Value::Map(hashmap! {
                1 => Value::Map(hashmap! {
                    1 => Value::Str(text.clone()) // 文本内容
                })
            })
        }
        MessageElement::Keyboard(keyboard_data) => {
            let rows_list: Vec<Value> = keyboard_data.rows.iter().map(|row| { // 每行
                let buttons_list: Vec<Value> = row.buttons.iter().map(|button| { // 每个按钮
                    Value::Map(hashmap! {
                        1 => Value::Str(button.id.clone()), // 按钮ID
                        2 => Value::Map(hashmap! { // 渲染信息
                            1 => Value::Str(button.render_data.label.clone()), // 点击前文本
                            2 => Value::Str(button.render_data.visited_label.clone()), // 点击后文本
                            3 => Value::Int(button.render_data.style as i64), // 按钮样式
                        }),
                        3 => Value::Map(hashmap! { // 操作信息
                            1 => Value::Int(button.action.action_type as i64), // 按钮类型
                            2 => Value::Map(hashmap! { // 权限
                                1 => Value::Int(button.action.permission.permission_type as i64), // 权限类型
                                2 => Value::List(button.action.permission.specify_role_ids.iter().map(|id| Value::Str(id.clone())).collect()), // 权限列表
                                3 => Value::List(button.action.permission.specify_user_ids.iter().map(|id| Value::Str(id.clone())).collect()), // 人员列表
                            }),
                            4 => Value::Str(button.action.unsupport_tips.clone()), // 不支持提示
                            5 => Value::Str(button.action.data.clone()), // 操作数据
                            7 => Value::Int(button.action.reply as i64), // 是否回复
                            8 => Value::Int(button.action.enter as i64), // 是否直接enter
                        })
                    })
                }).collect();

                Value::Map(hashmap! {
                    1 => Value::List(buttons_list)
                })
            }).collect();

            let button_data = hashmap! {
                1 => Value::Int(46),
                2 => Value::Map(hashmap! {
                    1 => Value::Map(hashmap! {
                        1 => Value::List(rows_list),
                        2 => Value::Str("1145140000".to_string()),
                    })
                }),
                3 => Value::Int(1)
            };

            Value::Map(hashmap! {
                53 => Value::Map(button_data)
            })
        }
    }
}

pub fn create_packet_hex(
    elements: Vec<MessageElement>,
    peer_type: String,
    peer_id: u64,
    seq: u64,
    random_number: u32 
) -> String {
    let content_list: Vec<Value> = elements.iter().map(element_to_protobuf_value).collect();

    let message = hashmap! {
        1 => Value::Map(if peer_type == "group" {
            hashmap! { 2 => Value::Map(hashmap! { 1 => Value::Int(peer_id as i64) }) }
        } else if peer_type == "user" {
            hashmap! { 1 => Value::Map(hashmap! { 1 => Value::Int(peer_id as i64) }) }
        } else {
            panic!("必须提供发送目标");
        }),
        2 => Value::Map(hashmap! {
            1 => Value::Int(1),
            2 => Value::Int(0),
            3 => Value::Int(0),
        }),
        3 => Value::Map(hashmap! {
            1 => Value::Map(hashmap! {
                2 => Value::List(content_list)
            })
        }),
        4 => Value::Int(seq as i64), // seq
        5 => Value::Int(random_number as i64), // 随机数
    };

    let encoded = ProtobufEncoder::encode(&message);
    hex::encode(encoded)
}