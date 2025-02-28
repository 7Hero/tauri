// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use super::MenuId;

/// Named images defined by the system.
#[cfg(target_os = "macos")]
#[cfg_attr(doc_cfg, doc(cfg(target_os = "macos")))]
#[derive(Debug, Clone)]
pub enum NativeImage {
  /// An add item template image.
  Add,
  /// Advanced preferences toolbar icon for the preferences window.
  Advanced,
  /// A Bluetooth template image.
  Bluetooth,
  /// Bookmarks image suitable for a template.
  Bookmarks,
  /// A caution image.
  Caution,
  /// A color panel toolbar icon.
  ColorPanel,
  /// A column view mode template image.
  ColumnView,
  /// A computer icon.
  Computer,
  /// An enter full-screen mode template image.
  EnterFullScreen,
  /// Permissions for all users.
  Everyone,
  /// An exit full-screen mode template image.
  ExitFullScreen,
  /// A cover flow view mode template image.
  FlowView,
  /// A folder image.
  Folder,
  /// A burnable folder icon.
  FolderBurnable,
  /// A smart folder icon.
  FolderSmart,
  /// A link template image.
  FollowLinkFreestanding,
  /// A font panel toolbar icon.
  FontPanel,
  /// A `go back` template image.
  GoLeft,
  /// A `go forward` template image.
  GoRight,
  /// Home image suitable for a template.
  Home,
  /// An iChat Theater template image.
  IChatTheater,
  /// An icon view mode template image.
  IconView,
  /// An information toolbar icon.
  Info,
  /// A template image used to denote invalid data.
  InvalidDataFreestanding,
  /// A generic left-facing triangle template image.
  LeftFacingTriangle,
  /// A list view mode template image.
  ListView,
  /// A locked padlock template image.
  LockLocked,
  /// An unlocked padlock template image.
  LockUnlocked,
  /// A horizontal dash, for use in menus.
  MenuMixedState,
  /// A check mark template image, for use in menus.
  MenuOnState,
  /// A MobileMe icon.
  MobileMe,
  /// A drag image for multiple items.
  MultipleDocuments,
  /// A network icon.
  Network,
  /// A path button template image.
  Path,
  /// General preferences toolbar icon for the preferences window.
  PreferencesGeneral,
  /// A Quick Look template image.
  QuickLook,
  /// A refresh template image.
  RefreshFreestanding,
  /// A refresh template image.
  Refresh,
  /// A remove item template image.
  Remove,
  /// A reveal contents template image.
  RevealFreestanding,
  /// A generic right-facing triangle template image.
  RightFacingTriangle,
  /// A share view template image.
  Share,
  /// A slideshow template image.
  Slideshow,
  /// A badge for a `smart` item.
  SmartBadge,
  /// Small green indicator, similar to iChat’s available image.
  StatusAvailable,
  /// Small clear indicator.
  StatusNone,
  /// Small yellow indicator, similar to iChat’s idle image.
  StatusPartiallyAvailable,
  /// Small red indicator, similar to iChat’s unavailable image.
  StatusUnavailable,
  /// A stop progress template image.
  StopProgressFreestanding,
  /// A stop progress button template image.
  StopProgress,

  /// An image of the empty trash can.
  TrashEmpty,
  /// An image of the full trash can.
  TrashFull,
  /// Permissions for a single user.
  User,
  /// User account toolbar icon for the preferences window.
  UserAccounts,
  /// Permissions for a group of users.
  UserGroup,
  /// Permissions for guests.
  UserGuest,
}

#[derive(Debug, Clone)]
pub enum MenuUpdate {
  /// Modifies the enabled state of the menu item.
  SetEnabled(bool),
  /// Modifies the title (label) of the menu item.
  SetTitle(String),
  /// Modifies the selected state of the menu item.
  SetSelected(bool),
  /// Update native image.
  #[cfg(target_os = "macos")]
  #[cfg_attr(doc_cfg, doc(cfg(target_os = "macos")))]
  SetNativeImage(NativeImage),
}

pub trait TrayHandle {
  fn set_icon(&self, icon: crate::Icon) -> crate::Result<()>;
  fn update_item(&self, id: u32, update: MenuUpdate) -> crate::Result<()>;
}

/// A window menu.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Menu<I: MenuId> {
  pub items: Vec<MenuEntry<I>>,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Submenu<I: MenuId> {
  pub title: String,
  pub enabled: bool,
  pub inner: Menu<I>,
}

impl<I: MenuId> Submenu<I> {
  /// Creates a new submenu with the given title and menu items.
  pub fn new<S: Into<String>>(title: S, menu: Menu<I>) -> Self {
    Self {
      title: title.into(),
      enabled: true,
      inner: menu,
    }
  }
}

impl<I: MenuId> Default for Menu<I> {
  fn default() -> Self {
    Self { items: Vec::new() }
  }
}

impl<I: MenuId> Menu<I> {
  /// Creates a new window menu.
  pub fn new() -> Self {
    Default::default()
  }

  /// Adds the custom menu item to the menu.
  pub fn add_item(mut self, item: CustomMenuItem<I>) -> Self {
    self.items.push(MenuEntry::CustomItem(item));
    self
  }

  /// Adds a native item to the menu.
  pub fn add_native_item(mut self, item: MenuItem) -> Self {
    self.items.push(MenuEntry::NativeItem(item));
    self
  }

  /// Adds an entry with submenu.
  pub fn add_submenu(mut self, submenu: Submenu<I>) -> Self {
    self.items.push(MenuEntry::Submenu(submenu));
    self
  }
}

/// A custom menu item.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CustomMenuItem<I: MenuId> {
  pub id: I,
  pub title: String,
  pub keyboard_accelerator: Option<String>,
  pub enabled: bool,
  pub selected: bool,
  #[cfg(target_os = "macos")]
  pub native_image: Option<NativeImage>,
}

impl<I: MenuId> CustomMenuItem<I> {
  /// Create new custom menu item.
  pub fn new<T: Into<String>>(id: I, title: T) -> Self {
    Self {
      id,
      title: title.into(),
      keyboard_accelerator: None,
      enabled: true,
      selected: false,
      #[cfg(target_os = "macos")]
      native_image: None,
    }
  }

  #[cfg(target_os = "macos")]
  pub fn native_image(mut self, image: NativeImage) -> Self {
    self.native_image.replace(image);
    self
  }

  /// Mark the item as disabled.
  pub fn disabled(mut self) -> Self {
    self.enabled = false;
    self
  }

  /// Mark the item as selected.
  pub fn selected(mut self) -> Self {
    self.selected = true;
    self
  }

  #[doc(hidden)]
  pub fn id_value(&self) -> u32 {
    let mut s = DefaultHasher::new();
    self.id.hash(&mut s);
    s.finish() as u32
  }
}

/// A system tray menu.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SystemTrayMenu<I: MenuId> {
  pub items: Vec<SystemTrayMenuEntry<I>>,
}

impl<I: MenuId> Default for SystemTrayMenu<I> {
  fn default() -> Self {
    Self { items: Vec::new() }
  }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SystemTraySubmenu<I: MenuId> {
  pub title: String,
  pub enabled: bool,
  pub inner: SystemTrayMenu<I>,
}

impl<I: MenuId> SystemTraySubmenu<I> {
  /// Creates a new submenu with the given title and menu items.
  pub fn new<S: Into<String>>(title: S, menu: SystemTrayMenu<I>) -> Self {
    Self {
      title: title.into(),
      enabled: true,
      inner: menu,
    }
  }
}

impl<I: MenuId> SystemTrayMenu<I> {
  /// Creates a new system tray menu.
  pub fn new() -> Self {
    Default::default()
  }

  /// Adds the custom menu item to the system tray menu.
  pub fn add_item(mut self, item: CustomMenuItem<I>) -> Self {
    self.items.push(SystemTrayMenuEntry::CustomItem(item));
    self
  }

  /// Adds a native item to the system tray menu.
  pub fn add_native_item(mut self, item: SystemTrayMenuItem) -> Self {
    self.items.push(SystemTrayMenuEntry::NativeItem(item));
    self
  }

  /// Adds an entry with submenu.
  pub fn add_submenu(mut self, submenu: SystemTraySubmenu<I>) -> Self {
    self.items.push(SystemTrayMenuEntry::Submenu(submenu));
    self
  }
}

/// An entry on the system tray menu.
#[derive(Debug, Clone)]
pub enum SystemTrayMenuEntry<I: MenuId> {
  /// A custom item.
  CustomItem(CustomMenuItem<I>),
  /// A native item.
  NativeItem(SystemTrayMenuItem),
  /// An entry with submenu.
  Submenu(SystemTraySubmenu<I>),
}

/// System tray menu item.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SystemTrayMenuItem {
  /// A separator.
  Separator,
}

/// An entry on the system tray menu.
#[derive(Debug, Clone)]
pub enum MenuEntry<I: MenuId> {
  /// A custom item.
  CustomItem(CustomMenuItem<I>),
  /// A native item.
  NativeItem(MenuItem),
  /// An entry with submenu.
  Submenu(Submenu<I>),
}

/// A menu item, bound to a pre-defined action or `Custom` emit an event. Note that status bar only
/// supports `Custom` menu item variants. And on the menu bar, some platforms might not support some
/// of the variants. Unsupported variant will be no-op on such platform.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MenuItem {
  /// Shows a standard "About" item
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Android / iOS:** Unsupported
  ///
  About(String),

  /// A standard "hide the app" menu item.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Android / iOS:** Unsupported
  ///
  Hide,

  /// A standard "Services" menu item.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Linux / Android / iOS:** Unsupported
  ///
  Services,

  /// A "hide all other windows" menu item.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Linux / Android / iOS:** Unsupported
  ///
  HideOthers,

  /// A menu item to show all the windows for this app.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Linux / Android / iOS:** Unsupported
  ///
  ShowAll,

  /// Close the current window.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Android / iOS:** Unsupported
  ///
  CloseWindow,

  /// A "quit this app" menu icon.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Android / iOS:** Unsupported
  ///
  Quit,

  /// A menu item for enabling copying (often text) from responders.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Android / iOS:** Unsupported
  ///
  Copy,

  /// A menu item for enabling cutting (often text) from responders.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Android / iOS:** Unsupported
  ///
  Cut,

  /// An "undo" menu item; particularly useful for supporting the cut/copy/paste/undo lifecycle
  /// of events.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Linux / Android / iOS:** Unsupported
  ///
  Undo,

  /// An "redo" menu item; particularly useful for supporting the cut/copy/paste/undo lifecycle
  /// of events.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Linux / Android / iOS:** Unsupported
  ///
  Redo,

  /// A menu item for selecting all (often text) from responders.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Android / iOS:** Unsupported
  ///
  SelectAll,

  /// A menu item for pasting (often text) into responders.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Android / iOS:** Unsupported
  ///
  Paste,

  /// A standard "enter full screen" item.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Linux / Android / iOS:** Unsupported
  ///
  EnterFullScreen,

  /// An item for minimizing the window with the standard system controls.
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Android / iOS:** Unsupported
  ///
  Minimize,

  /// An item for instructing the app to zoom
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Linux / Android / iOS:** Unsupported
  ///
  Zoom,

  /// Represents a Separator
  ///
  /// ## Platform-specific
  ///
  /// - **Windows / Android / iOS:** Unsupported
  ///
  Separator,
}
