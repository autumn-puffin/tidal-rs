use egui::WidgetText;
use tidal_rs::api::{
  modules::{
    collection_modules::{AlbumCollectionModule, AnyMediaCollectionModule, MixCollectionModule, PlaylistCollectionModule, TrackCollectionModule},
    HighlightModule, ModuleType,
  },
  Album, Artist, MediaType, Mix, Page, PageModule, Playlist, Track,
};

pub trait CustomUiExtension {
  fn kv_row(&mut self, kv: (impl Into<WidgetText>, impl Into<WidgetText>));
  fn shelf(&mut self, add_contents: impl FnOnce(&mut egui::Ui));
  fn card(&mut self, add_contents: impl FnOnce(&mut egui::Ui));
}
impl CustomUiExtension for egui::Ui {
  fn kv_row(&mut self, kv: (impl Into<WidgetText>, impl Into<WidgetText>)) {
    self.label(kv.0);
    self.label(kv.1);
    self.end_row();
  }
  fn shelf(&mut self, add_contents: impl FnOnce(&mut egui::Ui)) {
    self.horizontal(|ui| egui::ScrollArea::horizontal().show(ui, add_contents));
  }
  fn card(&mut self, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::popup(self.style()).show(self, |ui| {
      ui.vertical(|ui| {
        ui.set_width(200.);
        ui.set_height(200.);
        add_contents(ui);
      });
    });
  }
}

fn module_ui(ui: &mut egui::Ui, module_type: &ModuleType) {
  use ModuleType::*;
  match module_type {
    AlbumList(module) => ui.module_album_collection(module),
    HighlightModule(module) => ui.module_highlight(module),
    MixList(module) => ui.module_mix_collection(module),
    MixedTypesList(module) => ui.module_any_media_collection(module),
    PlaylistList(module) => ui.module_playlist_collection(module),
    TrackList(module) => ui.module_track_collection(module),
    _ => {
      ui.label("Unknown module type");
    }
  }
}

fn media_ui(ui: &mut egui::Ui, media_type: &MediaType) {
  use MediaType::*;
  match media_type {
    Album(media) => ui.media_album(media),
    Artist(media) => ui.media_artist(media),
    Playlist(media) => ui.media_playlist(media),
    Mix(media) => ui.media_mix(media),
    Track(media) => ui.media_track(media),
    _ => {
      ui.label("Unknown media type");
    }
  }
}

pub trait PageUiExtension {
  fn page_browser(&mut self, page: &Page);
  fn page_module(&mut self, module: &PageModule);
}
impl PageUiExtension for egui::Ui {
  fn page_browser(&mut self, page: &Page) {
    self.push_id("page_browser", |ui| {
      egui::ScrollArea::vertical().max_height(500.).min_scrolled_height(250.).show(ui, |ui| {
        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
          ui.heading(&page.title);
          let modules = page.rows.iter().flat_map(|row| row.modules.iter());
          for module in modules {
            ui.separator();
            ui.page_module(module);
          }
        });
      });
    });
  }
  fn page_module(&mut self, module: &PageModule) {
    let pre_title = module.pre_title.as_deref().unwrap_or_default();
    let title = module.title.as_deref().unwrap_or_default();
    let description = module.description.as_deref().unwrap_or_default();
    let id = &module.id;

    self.push_id(id, |ui| {
      if !pre_title.is_empty() {
        ui.label(pre_title);
      }
      if !title.is_empty() {
        ui.heading(title);
      }
      if !description.is_empty() {
        ui.label(description);
      }

      module_ui(ui, &module.r#type);
    });
  }
}

pub trait ModuleUiExtension {
  fn module_album_collection(&mut self, module: &AlbumCollectionModule);
  fn module_highlight(&mut self, module: &HighlightModule);
  fn module_mix_collection(&mut self, module: &MixCollectionModule);
  fn module_any_media_collection(&mut self, module: &AnyMediaCollectionModule);
  fn module_playlist_collection(&mut self, module: &PlaylistCollectionModule);
  fn module_track_collection(&mut self, module: &TrackCollectionModule);
}
impl ModuleUiExtension for egui::Ui {
  fn module_album_collection(&mut self, module: &AlbumCollectionModule) {
    let items = module
      .collection
      .paged_list
      .as_ref()
      .map_or(Default::default(), |list| list.paging.items.clone());
    self.shelf(|ui| {
      for item in items {
        ui.card(|ui| ui.media_album(&item));
      }
    });
  }
  fn module_highlight(&mut self, module: &HighlightModule) {
    // This ignores the playlist_style field for now
    // TODO: Implement playlist_style handling
    self.shelf(|ui| {
      for highlight in &module.highlights {
        ui.card(|ui| {
          ui.heading(&highlight.title);
          ui.separator();
          media_ui(ui, &highlight.item);
        });
      }
    });
  }
  fn module_mix_collection(&mut self, module: &MixCollectionModule) {
    let items = module
      .collection
      .paged_list
      .as_ref()
      .map_or(Default::default(), |list| list.paging.items.clone());
    self.shelf(|ui| {
      for item in items {
        ui.card(|ui| ui.media_mix(&item));
      }
    });
  }
  fn module_any_media_collection(&mut self, module: &AnyMediaCollectionModule) {
    let items = module
      .collection
      .paged_list
      .as_ref()
      .map_or(Default::default(), |list| list.paging.items.clone());
    self.shelf(|ui| {
      for item in items {
        ui.card(|ui| media_ui(ui, &item));
      }
    });
  }
  fn module_playlist_collection(&mut self, module: &PlaylistCollectionModule) {
    let items = module
      .collection
      .paged_list
      .as_ref()
      .map_or(Default::default(), |list| list.paging.items.clone());
    self.shelf(|ui| {
      for item in items {
        ui.card(|ui| ui.media_playlist(&item));
      }
    });
  }
  fn module_track_collection(&mut self, module: &TrackCollectionModule) {
    let items = module
      .collection
      .paged_list
      .as_ref()
      .map_or(Default::default(), |list| list.paging.items.clone());
    self.shelf(|ui| {
      for item in items {
        ui.card(|ui| ui.media_track(&item));
      }
    });
  }
}

pub trait MediaUiExtension {
  fn media_album(&mut self, media: &Album);
  fn media_artist(&mut self, media: &Artist);
  fn media_playlist(&mut self, media: &Playlist);
  fn media_mix(&mut self, media: &Mix);
  fn media_track(&mut self, media: &Track);
}
impl MediaUiExtension for egui::Ui {
  fn media_album(&mut self, media: &Album) {
    let title = &media.title;
    let artists = media.artists.as_ref().map_or("Unknown".to_owned(), |v| {
      v.iter().map(|a| a.name.as_str()).collect::<Vec<&str>>().join(", ")
    });
    self.heading(title);
    self.label(format!("By {}", artists));
  }
  fn media_artist(&mut self, media: &Artist) {
    let name = &media.name;
    self.heading(name);
  }
  fn media_playlist(&mut self, media: &Playlist) {
    let title = &media.title;
    let description = media.description.as_deref().unwrap_or_default();
    self.heading(title);
    self.label(description);
  }
  fn media_mix(&mut self, media: &Mix) {
    let title = media.title.as_deref().unwrap_or("Unnamed Mix");
    let sub_title = media.sub_title.as_deref().unwrap_or_default();
    self.heading(title);
    self.label(sub_title);
  }
  fn media_track(&mut self, media: &Track) {
    let title = &media.title;
    let artists = &media.artists.iter().map(|a| a.name.as_str()).collect::<Vec<&str>>().join(", ");
    self.heading(title);
    self.label(format!("By {}", artists));
  }
}
