use crate::core::{StyledNode, Display};

use super::{LayoutBox, BoxType, BoxProps};

pub fn create_layout_box<'a>(snode: StyledNode<'a>) -> LayoutBox<'a> {
  LayoutBox {
      box_type: match snode.display() {
          Display::Block => BoxType::BlockBox(BoxProps {
              node_type: snode.node_type,
              properties: snode.properties,
          }),
          Display::Inline => BoxType::InlineBox(BoxProps {
              node_type: snode.node_type,
              properties: snode.properties,
          }),
          Display::None => unreachable!(),
      },
      children: snode
          .children
          .into_iter()
          .fold(vec![], |mut acc: Vec<LayoutBox>, child| {
              match child.display() {
                  Display::Block => {
                      acc.push(create_layout_box(child));
                      acc
                  }
                  Display::Inline => {
                      match acc.last() {
                          Some(&LayoutBox {
                              box_type: BoxType::AnonymousBox,
                              ..
                          }) => {}
                          _ => acc.push(LayoutBox {
                              box_type: BoxType::AnonymousBox,
                              children: vec![],
                          }),
                      };
                      acc.last_mut().unwrap().children.push(create_layout_box(child));
                      acc
                  }
                  Display::None => unreachable!(),
              }
          }),
  }
}