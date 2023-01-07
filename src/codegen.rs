use crate::parse::{Node, NodeKind};

fn gen_lval(node: &Node) -> String {
    let mut result = String::new();
    if node.kind != NodeKind::LVar {
        panic!("代入の左辺値が変数ではありません");
    }
    result.push_str("  mov rax, rbp\n");
    result.push_str(&format!("  sub rax, {}\n", node.offset));
    result.push_str("  push rax\n");

    result
}

pub fn gen(node: &Node) -> String {
    let mut result = String::new();
    match node.kind {
        NodeKind::Num => {
            result.push_str(&format!("  push {}\n", node.val));
            return result;
        }
        NodeKind::LVar => {
            result.push_str(gen_lval(node).as_str());
            result.push_str("  pop rax\n");
            result.push_str("  mov rax, [rax]\n");
            result.push_str("  push rax\n");
            return result;
        }
        NodeKind::Assign => {
            result.push_str(gen_lval(node.lhs.as_ref().unwrap()).as_str());
            result.push_str(gen(node.rhs.as_ref().unwrap()).as_str());
            result.push_str("  pop rdi\n");
            result.push_str("  pop rax\n");
            result.push_str("  mov [rax], rdi\n");
            result.push_str("  push rdi\n");
            return result;
        }
        _ => {}
    };

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
