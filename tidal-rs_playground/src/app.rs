use crate::Event;
use std::sync::{mpsc, Arc, Mutex};
use tidal_rs::client::Client;

mod custom_ui;
use custom_ui::*;

pub struct App {
  event_sender: mpsc::Sender<Event>,
  client: Arc<Mutex<Client>>,
}
impl App {
  pub fn new(event_sender: mpsc::Sender<Event>, client: Arc<Mutex<Client>>) -> Self {
    Self { event_sender, client }
  }
}
impl App {
  fn draw_auth_panel(&self, ui: &mut egui::Ui) {
    ui.label("Auth Panel");
    ui.button("Auth with Device Flow")
      .on_hover_text("Authenticates with the device flow")
      .clicked()
      .then(|| self.event_sender.send(Event::AuthWithDeviceFlow).unwrap());
    egui::ScrollArea::horizontal().max_width(250.).show(ui, |ui| {
      egui::Grid::new("auth_grid").num_columns(2).striped(true).show(ui, |ui| {
        let client = self.client.lock().unwrap();
        kv_row(ui, ("Client ID", client.get_client_credentials().id()));
        kv_row(ui, ("Client Secret", client.get_client_credentials().secret()));

        if let Some(auth_creds) = client.get_auth_credentials() {
          kv_row(ui, ("Access Token", auth_creds.access_token()));
          kv_row(ui, ("Refresh Token", auth_creds.refresh_token().unwrap_or("None")));
          kv_row(ui, ("Expires At", auth_creds.expires_at().to_string()));
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
    });
  }
}
impl eframe::App for App {
  fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
      ui.horizontal(|ui| {
        ui.label("TOP PANEL");
      });
    });
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
      ui.label("LEFT PANEL");
    });
    egui::SidePanel::right("auth_panel").show(ctx, |ui| self.draw_auth_panel(ui));
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.label("CENTRAL PANEL");
    });
  }
}
