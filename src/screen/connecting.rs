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
use crate::screen::{Screen, ScreenSystem};
use crate::ui;
use std::sync::Arc;

pub struct Connecting {
    elements: Option<UIElements>,
    target: String,
}

impl Clone for Connecting {
    fn clone(&self) -> Self {
        Connecting {
            elements: None,
            target: self.target.clone(),
        }
    }
}

struct UIElements {
    logo: ui::logo::Logo,
    _connect_msg: ui::TextRef,
    _msg: ui::TextRef,
    _disclaimer: ui::TextRef,
}

impl Connecting {
    pub fn new(target: &str) -> Connecting {
        Connecting {
            elements: None,
            target: target.to_owned(),
        }
    }
}

impl super::Screen for Connecting {
    fn on_active(
        &mut self,
        _screen_sys: &ScreenSystem,
        renderer: Arc<render::Renderer>,
        ui_container: &mut ui::Container,
    ) {
        let logo = ui::logo::Logo::new(renderer.resources.clone(), ui_container);

        let connect_msg = ui::TextBuilder::new()
            .text("Connecting to")
            .position(0.0, -16.0)
            .alignment(ui::VAttach::Middle, ui::HAttach::Center)
            .create(ui_container);

        let msg = ui::TextBuilder::new()
            .text(self.target.clone())
            .position(0.0, 16.0)
            .colour((255, 255, 85, 255))
            .alignment(ui::VAttach::Middle, ui::HAttach::Center)
            .create(ui_container);

        // Disclaimer
        let disclaimer = ui::TextBuilder::new()
            .text("Not affiliated with Mojang/Minecraft")
            .position(5.0, 5.0)
            .colour((255, 200, 200, 255))
            .alignment(ui::VAttach::Bottom, ui::HAttach::Right)
            .create(ui_container);

        self.elements = Some(UIElements {
            logo,
            _disclaimer: disclaimer,
            _msg: msg,
            _connect_msg: connect_msg,
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

    fn clone_screen(&self) -> Box<dyn Screen> {
        Box::new(self.clone())
    }
}
