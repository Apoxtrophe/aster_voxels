mod config;
use bevy_flycam::NoCameraPlayerPlugin;
use config::*;

mod player;
use player::*;
use bevy::prelude::*;

use bevy::window::{Window, PresentMode, CursorIcon, CursorGrabMode, WindowResolution};
use bevy::window::PrimaryWindow;
