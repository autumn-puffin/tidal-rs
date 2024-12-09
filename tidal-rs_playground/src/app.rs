use crate::{AppEvent, BackgroundEvent};
use egui::{Label, RichText, TextEdit};
use std::sync::{mpsc, Arc, Mutex};
use tidal_rs::client::Client;

mod custom_ui;
use custom_ui::*;

pub struct App {
  event_sender: mpsc::Sender<BackgroundEvent>,
  event_receiver: mpsc::Receiver<AppEvent>,
  client: Arc<Mutex<Client>>,
  app_state: AppState,
}
impl App {
  pub fn new(event_sender: mpsc::Sender<BackgroundEvent>, event_receiver: mpsc::Receiver<AppEvent>, client: Arc<Mutex<Client>>) -> Self {
    Self {
      event_sender,
      event_receiver,
      client,
      app_state: AppState::default(),
    }
  }
}
// Event Methods
impl App {
  fn handle_events(&mut self) {
    while let Ok(event) = self.event_receiver.try_recv() {
      match event {
        AppEvent::SetCataloguePage(page) => self.app_state.interface_state.page = Some(format!("{page:#?}")),
      }
    }
  }
}
// UI Methods
impl App {
  fn draw_auth_panel(&self, ui: &mut egui::Ui) {
    ui.label("Auth Panel");
    ui.button("Auth with Device Flow")
      .on_hover_text("Authenticates with the device flow")
      .clicked()
      .then(|| self.event_sender.send(BackgroundEvent::AuthWithDeviceFlow).unwrap());
    egui::ScrollArea::horizontal().max_width(250.).show(ui, |ui| self.auth_grid(ui));
  }
  fn auth_grid(&self, ui: &mut egui::Ui) {
    egui::Grid::new("auth_grid").num_columns(2).striped(true).show(ui, |ui| {
      let client = self.client.lock().unwrap();
      kv_row(ui, ("Client ID", client.get_client_credentials().id()));
      kv_row(ui, ("Client Secret", client.get_client_credentials().secret()));

      if let Some(auth_creds) = client.get_auth_credentials() {
        kv_row(ui, ("Access Token", auth_creds.access_token()));
        kv_row(ui, ("Refresh Token", auth_creds.refresh_token().unwrap_or("None")));
        let expires_at = chrono::DateTime::from_timestamp(auth_creds.expires_at(), 0)
          .map(|dt| dt.to_string())
          .unwrap_or("None".to_string());
        kv_row(ui, ("Expires At", expires_at));
        if let Some(user) = auth_creds.auth_user() {
          kv_row(ui, ("Username", &user.username));
          kv_row(ui, ("User ID", user.user_id.to_string()));
          kv_row(ui, ("User Email", &user.email));
          kv_row(ui, ("User Country", user.country_code.name()));
        } else {
          ui.label("No User");
          ui.end_row();
        }
      } else {
        ui.label("No Auth Creds");
        ui.end_row();
      }
    });
  }

  fn draw_selection_panel(&mut self, ui: &mut egui::Ui) {
    let interface_state = &mut self.app_state.interface_state;
    ui.vertical(|ui| {
      ui.selectable_value(&mut interface_state.selection, Some(InterfaceSelection::Users), "Users");
      ui.selectable_value(&mut interface_state.selection, Some(InterfaceSelection::Catalogue), "Catalogue");
      ui.selectable_value(&mut interface_state.selection, Some(InterfaceSelection::Track), "Track");
    });
  }

  fn draw_interactions_panel(&mut self, ui: &mut egui::Ui) {
    let selection = self.app_state.interface_state.selection;
    egui::ScrollArea::vertical().show(ui, |ui| match selection {
      Some(InterfaceSelection::Users) => {
        ui.label(RichText::new("Users").heading());
        ui.separator();
      }
      Some(InterfaceSelection::Catalogue) => self.catalogue_interactions(ui),
      Some(InterfaceSelection::Track) => {
        ui.label(RichText::new("Track").heading());
        ui.separator();
      }
      None => {
        ui.label(RichText::new("tidal-rs playground").heading());
        ui.separator();
        ui.label("Select an interface to interact with");
      }
    });
  }
  fn catalogue_interactions(&mut self, ui: &mut egui::Ui) {
    let interface_state = &mut self.app_state.interface_state;
    ui.label(RichText::new("Catalogue").heading());
    ui.separator();
    ui.label("Get Page");
    ui.horizontal(|ui| {
      ui.add(TextEdit::singleline(&mut interface_state.page_path).desired_width(100.));
      ui.button("Get").on_hover_text("Get the specified page").clicked().then(|| {
        self
          .event_sender
          .send(BackgroundEvent::CatalogueGetPage(interface_state.page_path.clone()))
          .unwrap();
      })
    });
    egui::ScrollArea::vertical().max_height(250.).min_scrolled_height(250.).show(ui, |ui| {
      egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        let page = match &interface_state.page {
          Some(page) => page,
          None => "No page",
        };
        ui.add(Label::new(page));
      });
    });
  }
}
impl eframe::App for App {
  fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
    self.handle_events();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
      ui.horizontal(|ui| {
        ui.label("TOP PANEL");
      });
    });
    egui::SidePanel::left("selection_panel").show(ctx, |ui| self.draw_selection_panel(ui));
    egui::SidePanel::right("auth_panel").show(ctx, |ui| self.draw_auth_panel(ui));
    egui::CentralPanel::default().show(ctx, |ui| self.draw_interactions_panel(ui));
  }
}

#[derive(Default)]
struct AppState {
  interface_state: InterfaceState,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum InterfaceSelection {
  Users,
  Catalogue,
  Track,
}

#[derive(Default)]
struct InterfaceState {
  selection: Option<InterfaceSelection>,
  // Catalogue
  page_path: String,
  page: Option<String>,
}
