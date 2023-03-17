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

use crate::{ui, Game};

use crate::render::Renderer;
use crate::screen::{Screen, ScreenSystem};
use std::rc::Rc;
use std::sync::Arc;

type EditAccountEntryCallback = dyn Fn(&mut Game, String, String);

pub struct EditAccountEntry {
    elements: Option<UIElements>,
    entry_info: Option<(String, String)>,
    done_callback: Rc<EditAccountEntryCallback>, // game, name, password
}

impl Clone for EditAccountEntry {
    fn clone(&self) -> Self {
        Self {
            elements: None,
            entry_info: self.entry_info.clone(),
            done_callback: self.done_callback.clone(),
        }
    }
}

struct UIElements {
    logo: ui::logo::Logo,

    _name: ui::TextBoxRef,
    _password: ui::TextBoxRef,
    _done: ui::ButtonRef,
    _cancel: ui::ButtonRef,
}

impl EditAccountEntry {
    pub fn new(
        entry_info: Option<(String, String)>,
        done_callback: Rc<EditAccountEntryCallback>,
    ) -> Self {
        Self {
            elements: None,
            entry_info,
            done_callback,
        }
    }
}

impl super::Screen for EditAccountEntry {
    fn on_active(
        &mut self,
        _screen_sys: &ScreenSystem,
        renderer: Arc<Renderer>,
        ui_container: &mut ui::Container,
    ) {
        let logo = ui::logo::Logo::new(renderer.resources.clone(), ui_container);

        // Name
        let account_name = ui::TextBoxBuilder::new()
            .input(self.entry_info.as_ref().map_or("", |v| &v.0))
            .position(0.0, -20.0)
            .size(400.0, 40.0)
            .alignment(ui::VAttach::Middle, ui::HAttach::Center)
            .create(ui_container);
        ui::TextBox::make_focusable(&account_name, ui_container);
        ui::TextBuilder::new()
            .text("Name:")
            .position(0.0, -18.0)
            .attach(&mut *account_name.borrow_mut());

        // Address
        let account_password = ui::TextBoxBuilder::new()
            .input(self.entry_info.as_ref().map_or("", |v| &v.1))
            .position(0.0, 40.0)
            .size(400.0, 40.0)
            .password(true)
            .alignment(ui::VAttach::Middle, ui::HAttach::Center)
            .create(ui_container);
        ui::TextBox::make_focusable(&account_password, ui_container);
        ui::TextBuilder::new()
            .text("Password:")
            .position(0.0, -18.0)
            .attach(&mut *account_password.borrow_mut());

        // Done
        let done = ui::ButtonBuilder::new()
            .position(110.0, 100.0)
            .size(200.0, 40.0)
            .alignment(ui::VAttach::Middle, ui::HAttach::Center)
            .create(ui_container);
        {
            let mut done = done.borrow_mut();
            let txt = ui::TextBuilder::new()
                .text("Done")
                .alignment(ui::VAttach::Middle, ui::HAttach::Center)
                .attach(&mut *done);
            done.add_text(txt);
            let account_name = account_name.clone();
            let account_password = account_password.clone();
            let callback = self.done_callback.clone();
            done.add_click_func(move |_, game| {
                (*callback.clone())(
                    game,
                    account_name.borrow().input.clone(),
                    account_password.borrow().input.clone(),
                );
                game.screen_sys.clone().pop_screen();
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
            cancel.add_click_func(|_, game| {
                game.screen_sys.clone().pop_screen();
                true
            });
        }

        self.elements = Some(UIElements {
            logo,
            _name: account_name,
            _password: account_password,
            _done: done,
            _cancel: cancel,
        });
    }

    fn on_deactive(
        &mut self,
        _screen_sys: &ScreenSystem,
        _renderer: Arc<Renderer>,
        _ui_container: &mut ui::Container,
    ) {
        // Clean up
        self.elements = None
    }

    fn tick(
        &mut self,
        _screen_sys: &ScreenSystem,
        renderer: Arc<Renderer>,
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
