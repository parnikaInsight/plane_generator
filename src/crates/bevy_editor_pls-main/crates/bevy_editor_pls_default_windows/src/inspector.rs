use crate::hierarchy::SelectedEntities;

use super::add::{AddWindow, AddWindowState};
use super::hierarchy::HierarchyWindow;
use bevy::prelude::{Entity, Mut, World};
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::egui;
use bevy_inspector_egui::{world_inspector::WorldUIContext, WorldInspectorParams};

pub struct InspectorWindow;
impl EditorWindow for InspectorWindow {
    type State = ();
    const NAME: &'static str = "Inspector";

    fn ui(world: &mut World, cx: EditorWindowContext, ui: &mut egui::Ui) {
        let inspected = &cx.state::<HierarchyWindow>().unwrap().selected;

        let add_window_state = cx.state::<AddWindow>();
        inspector(world, inspected, ui, add_window_state);
    }
}

fn inspector(
    world: &mut World,
    inspected: &SelectedEntities,
    ui: &mut egui::Ui,
    add_window_state: Option<&AddWindowState>,
) {
    if inspected.is_empty() {
        ui.label("No entity selected");
        return;
    }

    world.resource_scope(|world, params: Mut<WorldInspectorParams>| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut inspector_ui = InspectorUi::new(world, &*params, add_window_state);

            match inspected.len() {
                1 => {
                    let inspected = inspected.iter().next().unwrap();
                    inspector_ui.entity(ui, inspected);
                }
                _ => {
                    ui.label("Inspector for multiple entities not yet implemented.");
                }
            };
        });
    });
}

struct InspectorUi<'a> {
    world: &'a mut World,
    params: &'a WorldInspectorParams,
    add_window_state: Option<&'a AddWindowState>,
}
impl<'a> InspectorUi<'a> {
    fn new(
        world: &'a mut World,
        params: &'a WorldInspectorParams,
        add_window_state: Option<&'a AddWindowState>,
    ) -> Self {
        Self {
            world,
            params,
            add_window_state,
        }
    }

    fn add_ui(&mut self, ui: &mut egui::Ui, entity: Entity) {
        if let Some(add_window_state) = self.add_window_state {
            let layout = egui::Layout::top_down(egui::Align::Center).with_cross_justify(true);
            ui.with_layout(layout, |ui| {
                ui.menu_button("+", |ui| {
                    if let Some(add_item) = crate::add::add_ui(ui, add_window_state) {
                        add_item.add_to_entity(self.world, entity);
                    }
                });
            });
        }
    }

    fn components_ui(&mut self, ui: &mut egui::Ui, entity: Entity) {
        let id = egui::Id::new("inspector");
        let mut world_ui_ctx = WorldUIContext::new(self.world, None);
        world_ui_ctx.component_kind_ui(
            ui,
            |archetype| archetype.table_components(),
            "Components",
            entity,
            self.params,
            id,
        );
        world_ui_ctx.component_kind_ui(
            ui,
            |archetype| archetype.sparse_set_components(),
            "Components (Sparse)",
            entity,
            self.params,
            id,
        );
    }

    fn entity(&mut self, ui: &mut egui::Ui, entity: Entity) {
        self.components_ui(ui, entity);
        self.add_ui(ui, entity);
    }
}

pub fn label_button(ui: &mut egui::Ui, text: &str, text_color: egui::Color32) -> bool {
    ui.add(egui::Button::new(egui::RichText::new(text).color(text_color)).frame(false))
        .clicked()
}
