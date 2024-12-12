use crate::{AppEvent, BackgroundEvent};
use egui::{Label, RichText, TextEdit};
use std::sync::{mpsc, Arc, Mutex};
use tidal_rs::{api::Page, client::Client};

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
        AppEvent::SetCataloguePage(page) => {
          self.app_state.interface_state.page_text = format!("{page:#?}");
          self.app_state.interface_state.page = Some(page);
        }
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
      ui.kv_row(("Client ID", client.get_client_credentials().id()));
      ui.kv_row(("Client Secret", client.get_client_credentials().secret()));

      if let Some(auth_creds) = client.get_auth_credentials() {
        ui.kv_row(("Access Token", auth_creds.access_token()));
        ui.kv_row(("Refresh Token", auth_creds.refresh_token().unwrap_or("None")));
        let expires_at = chrono::DateTime::from_timestamp(auth_creds.expires_at(), 0)
          .map(|dt| dt.to_string())
          .unwrap_or("None".to_string());
        ui.kv_row(("Expires At", expires_at));
        if let Some(user) = auth_creds.auth_user() {
          ui.kv_row(("Username", &user.username));
          ui.kv_row(("User ID", user.user_id.to_string()));
          ui.kv_row(("User Email", &user.email));
          ui.kv_row(("User Country", user.country_code.name()));
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
        ui.add(Label::new(&interface_state.page_text));
      });
    });
    if let Some(page) = &interface_state.page {
      ui.heading("Page Browser");
      ui.separator();
      ui.page_browser(page);
    }
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

struct InterfaceState {
  selection: Option<InterfaceSelection>,
  // Catalogue
  page_path: String,
  page_text: String,
  page: Option<Page>,
}

impl Default for InterfaceState {
  fn default() -> Self {
    Self {
      selection: None,
      page_path: Default::default(),
      page_text: "No Page".to_owned(),
      page: None,
    }
  }
}
