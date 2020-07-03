/*!
Make sure that the Repeated expression are just components without any chilodren
 */

use crate::diagnostics::Diagnostics;

use crate::{object_tree::*, typeregister::Type};
use std::{cell::RefCell, rc::Rc};

pub fn create_repeater_components(component: &Rc<Component>, _diag: &mut Diagnostics) {
    recurse_elem(&component.root_element, &(), &mut |elem, _| {
        if elem.borrow().repeated.is_none() {
            return;
        }
        let parent_element = Rc::downgrade(elem);
        let mut elem = elem.borrow_mut();

        let comp = Rc::new(Component {
            root_element: Rc::new(RefCell::new(Element {
                id: elem.id.clone(),
                base_type: std::mem::take(&mut elem.base_type),
                bindings: std::mem::take(&mut elem.bindings),
                children: std::mem::take(&mut elem.children),
                property_declarations: std::mem::take(&mut elem.property_declarations),
                property_animations: std::mem::take(&mut elem.property_animations),
                repeated: None,
                node: elem.node.clone(),
                enclosing_component: Default::default(),
            })),
            parent_element,
            ..Component::default()
        });

        let weak = Rc::downgrade(&comp);
        recurse_elem(&comp.root_element, &(), &mut |e, _| {
            e.borrow_mut().enclosing_component = weak.clone()
        });
        create_repeater_components(&comp, _diag);
        elem.base_type = Type::Component(comp);
    });
}
