use std::sync::{mpsc, Arc, Mutex};

use tidal_rs::client::Client;

use crate::Event;

pub struct App {
  event_sender: mpsc::Sender<Event>,
  client: Arc<Mutex<Client>>,
}
impl App {
  pub fn new(event_sender: mpsc::Sender<Event>, client: Arc<Mutex<Client>>) -> Self {
    Self { event_sender, client }
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
    egui::SidePanel::right("auth_panel").show(ctx, |ui| {
      ui.label("Auth Panel");
      ui.button("Auth with Device Flow")
        .on_hover_text("Authenticates with the device flow")
        .clicked()
        .then(|| self.event_sender.send(Event::AuthWithDeviceFlow).unwrap());
      egui::ScrollArea::horizontal().max_width(250.).show(ui, |ui| {
        egui::Grid::new("auth_grid").num_columns(2).striped(true).show(ui, |ui| {
          let client = self.client.lock().unwrap();
          ui.label("Client ID");
          ui.label(client.get_client_credentials().id());
          ui.end_row();
          ui.label("Client Secret");
          ui.label(client.get_client_credentials().secret());
          ui.end_row();

          if let Some(auth_creds) = client.get_auth_credentials() {
            ui.label("Access Token");
            ui.label(auth_creds.access_token());
            ui.end_row();
            ui.label("Refresh Token");
            ui.label(auth_creds.refresh_token().unwrap_or("None"));
            ui.end_row();
            ui.label("Expires At");
            ui.label(auth_creds.expires_at().to_string());
            ui.end_row();
          } else {
            ui.label("Auth Creds");
            ui.label("No Auth Creds");
            ui.end_row();
          }
        });
      });
    });
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.label("CENTRAL PANEL");
    });
  }
}
