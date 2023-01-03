use crate::parse::{Node, NodeKind};

pub fn gen(node: &Node) -> String {
    let mut result = String::new();
    if node.kind == NodeKind::Num {
        result.push_str(&format!("  push {}\n", node.val));
        return result;
    }

    result.push_str(
        gen(match &node.lhs {
            Some(v) => v,
            None => {
                return result;
            }
        })
        .as_str(),
    );
    result.push_str(
        gen(match &node.rhs {
            Some(v) => v,
            None => {
                return result;
            }
        })
        .as_str(),
    );

    result.push_str("  pop rdi\n");
    result.push_str("  pop rax\n");

    match node.kind {
        NodeKind::Add => result.push_str("  add rax, rdi\n"),
        NodeKind::Sub => result.push_str("  sub rax, rdi\n"),
        NodeKind::Mul => result.push_str("  imul rax, rdi\n"),
        NodeKind::Div => {
            result.push_str("  cqo\n");
            result.push_str("  idiv rdi\n");
        }
        NodeKind::Eq => {
            result.push_str("  cmp rax, rdi\n");
            result.push_str("  sete al\n");
            result.push_str("  movzb rax, al\n");
        }
        NodeKind::Ne => {
            result.push_str("  cmp rax, rdi\n");
            result.push_str("  setne al\n");
            result.push_str("  movzb rax, al\n");
        }
        NodeKind::Lt => {
            result.push_str("  cmp rax, rdi\n");
            result.push_str("  setl al\n");
            result.push_str("  movzb rax, al\n");
        }
        NodeKind::Le => {
            result.push_str("  cmp rax, rdi\n");
            result.push_str("  setle al\n");
            result.push_str("  movzb rax, al\n");
        }
        _ => {}
    };

    result.push_str("  push rax\n");
    result
}
