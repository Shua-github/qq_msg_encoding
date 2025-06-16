#[cfg(test)]
use crate::core::{KeyboardData, Row, Button, RenderData, Action, Permission, MessageElement, create_packet_hex};
use crate::rng::SimpleRng;

#[test]
fn test_packet_with_multiple_elements() {
    let mut rng = SimpleRng::new(12345114);
    let keyboard_data = KeyboardData {
        rows: vec![Row {
            buttons: vec![Button {
                id: rng.gen_uuid_v4(),
                render_data: RenderData {
                    label: "点击我".into(),
                    visited_label: "已点击".into(),
                    style: 1,
                },
                action: Action {
                    action_type: 0,
                    permission: Permission {
                        permission_type: 2,
                        specify_role_ids: vec![],
                        specify_user_ids: vec![],
                    },
                    unsupport_tips: "不支持".into(),
                    data: "https://example.com".into(),
                    reply: true,
                    enter: false,
                }
            }]
        }]
    };

    let elements = vec![
        MessageElement::Keyboard(keyboard_data),
    ];

    let hex_string = create_packet_hex(elements, "group".to_string(), 260011598, rng.next_u32() as u64,rng.next_u32());
    println!("{}", hex_string);
}