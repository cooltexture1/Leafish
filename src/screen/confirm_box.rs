// This file is part of Leafish.
//
// Leafish is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// Leafish is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Leafish. If not, see <https://www.gnu.org/licenses/>. 

use crate::render;
use crate::ui;
use crate::Game;

use crate::screen::{Screen, ScreenSystem};
use std::rc::Rc;
use std::sync::Arc;

pub struct ConfirmBox {
    elements: Option<UIElements>,
    prompt: String,
    cancel_callback: Rc<dyn Fn(&mut Game)>,
    confirm_callback: Rc<dyn Fn(&mut Game)>,
}

impl Clone for ConfirmBox {
    fn clone(&self) -> Self {
        Self {
            elements: None,
            prompt: self.prompt.clone(),
            cancel_callback: self.cancel_callback.clone(),
            confirm_callback: self.confirm_callback.clone(),
        }
    }
}

struct UIElements {
    logo: ui::logo::Logo,

    _prompt: ui::TextRef,
    _confirm: ui::ButtonRef,
    _cancel: ui::ButtonRef,
}

impl ConfirmBox {
    pub fn new(
        prompt: String,
        cancel_callback: Rc<dyn Fn(&mut Game)>,
        confirm_callback: Rc<dyn Fn(&mut Game)>,
    ) -> Self {
        Self {
            elements: None,
            prompt,
            cancel_callback,
            confirm_callback,
        }
    }
}

impl super::Screen for ConfirmBox {
    fn on_active(
        &mut self,
        _screen_sys: &ScreenSystem,
        renderer: Arc<render::Renderer>,
        ui_container: &mut ui::Container,
    ) {
        let logo = ui::logo::Logo::new(renderer.resources.clone(), ui_container);

        // Prompt
        let prompt = ui::TextBuilder::new()
            .text(self.prompt.clone())
            .position(0.0, 40.0)
            .alignment(ui::VAttach::Middle, ui::HAttach::Center)
            .create(ui_container);

        // Confirm
        let confirm = ui::ButtonBuilder::new()
            .position(110.0, 100.0)
            .size(200.0, 40.0)
            .alignment(ui::VAttach::Middle, ui::HAttach::Center)
            .create(ui_container);
        {
            let mut confirm = confirm.borrow_mut();
            let txt = ui::TextBuilder::new()
                .text("Confirm")
                .alignment(ui::VAttach::Middle, ui::HAttach::Center)
                .attach(&mut *confirm);
            confirm.add_text(txt);
            let callback = self.confirm_callback.clone();
            confirm.add_click_func(move |_, game| {
                (*callback)(game);
                true
            });
        }

        // Cancel
        let cancel = ui::ButtonBuilder::new()
            .position(-110.0, 100.0)
            .size(200.0, 40.0)
            .alignment(ui::VAttach::Middle, ui::HAttach::Center)
            .create(ui_container);
        {
            let mut cancel = cancel.borrow_mut();
            let txt = ui::TextBuilder::new()
                .text("Cancel")
                .alignment(ui::VAttach::Middle, ui::HAttach::Center)
                .attach(&mut *cancel);
            cancel.add_text(txt);
            let callback = self.cancel_callback.clone();
            cancel.add_click_func(move |_, game| {
                (*callback)(game);
                true
            });
        }

        self.elements = Some(UIElements {
            logo,
            _prompt: prompt,
            _confirm: confirm,
            _cancel: cancel,
        });
    }

    fn on_deactive(
        &mut self,
        _screen_sys: &ScreenSystem,
        _renderer: Arc<render::Renderer>,
        _ui_container: &mut ui::Container,
    ) {
        // Clean up
        self.elements = None
    }

    fn tick(
        &mut self,
        _screen_sys: &ScreenSystem,
        renderer: Arc<render::Renderer>,
        _ui_container: &mut ui::Container,
        _delta: f64,
    ) {
        let elements = self.elements.as_mut().unwrap();
        elements.logo.tick(renderer);
    }

    fn is_closable(&self) -> bool {
        true
    }

    fn clone_screen(&self) -> Box<dyn Screen> {
        Box::new(self.clone())
    }
}
