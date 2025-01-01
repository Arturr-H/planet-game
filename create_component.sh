#!/bin/bash

# Check if a name is provided
if [ -z "$1" ]; then
    echo "Usage:"
    echo "  $0 <component_name>"
    echo "  $0 --add <file_name> <folder_name>"
    echo "  $0 --folder <folder_path> <component_name>"
    exit 1
fi

snake_to_camel() {
    local input="$1"
    echo "$input" | awk -F'_' '{ 
        for (i=1; i<=NF; i++) {
            printf toupper(substr($i,1,1)) tolower(substr($i,2))
        }
        printf "\n"
    }'
}

# Default folder
BASE_FOLDER="components"

# Parse --folder flag
if [ "$1" == "--folder" ]; then
    if [ -z "$2" ] || [ -z "$3" ]; then
        echo "Usage: $0 --folder <folder_path> <component_name>"
        exit 1
    fi
    BASE_FOLDER="$2"
    ORIGINAL_NAME="$3"
elif [ "$1" == "--add" ]; then
    if [ -z "$2" ] || [ -z "$3" ]; then
        echo "Usage: $0 --add <file_name> <folder_name>"
        exit 1
    fi

    FILE_NAME="$2"
    FOLDER_NAME=$(echo "$3" | tr '[:upper:]' '[:lower:]')
    FILE_PATH="src/$BASE_FOLDER/$FOLDER_NAME/$FILE_NAME.rs"

    COMPONENT_NAME_LOWER=$(echo "$FILE_NAME" | tr '[:upper:]' '[:lower:]')
    COMPONENT_NAME_FORMATTED=$(snake_to_camel "$COMPONENT_NAME_LOWER")
    COMPONENT_PLUGIN_NAME="${COMPONENT_NAME_FORMATTED}Plugin"

    # Ensure the folder exists
    if [ ! -d "src/$BASE_FOLDER/$FOLDER_NAME" ]; then
        echo "Error: Folder 'src/$BASE_FOLDER/$FOLDER_NAME' does not exist."
        exit 1
    fi

    # Create the file
    echo "/* Imports */
use bevy::prelude::*;

/* Constants */

/// $COMPONENT_NAME_FORMATTED component
#[derive(Component)]
pub struct $COMPONENT_NAME_FORMATTED;

pub struct $COMPONENT_PLUGIN_NAME;
impl Plugin for $COMPONENT_PLUGIN_NAME {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, $COMPONENT_NAME_FORMATTED::setup)
            .add_systems(Update, $COMPONENT_NAME_FORMATTED::update);
    }
}

impl $COMPONENT_NAME_FORMATTED {
    pub fn setup(mut commands: Commands) -> () {}
    pub fn update() -> () {}
}" > "$FILE_PATH"
    echo "File '$FILE_PATH' created successfully."
    exit 0
else
    ORIGINAL_NAME="$1"
fi

# Original input and transformations
COMPONENT_NAME_LOWER=$(echo "$ORIGINAL_NAME" | tr '[:upper:]' '[:lower:]')
COMPONENT_NAME_FORMATTED=$(snake_to_camel "$ORIGINAL_NAME")
COMPONENT_PLUGIN_NAME="${COMPONENT_NAME_FORMATTED}Plugin"

COMPONENT_DIR="src/$BASE_FOLDER/$COMPONENT_NAME_LOWER"
MOD_FILE="$COMPONENT_DIR/mod.rs"
COMPONENT_FILE="$COMPONENT_DIR/$COMPONENT_NAME_LOWER.rs"

# Create directory and files
mkdir -p "$COMPONENT_DIR"

echo "pub mod $COMPONENT_NAME_LOWER;" > "$MOD_FILE"

echo "/* Imports */
use bevy::prelude::*;

/* Constants */

/// $COMPONENT_NAME_FORMATTED component
#[derive(Component)]
pub struct $COMPONENT_NAME_FORMATTED;

pub struct $COMPONENT_PLUGIN_NAME;
impl Plugin for $COMPONENT_PLUGIN_NAME {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, $COMPONENT_NAME_FORMATTED::setup)
            .add_systems(Update, $COMPONENT_NAME_FORMATTED::update);
    }
}

impl $COMPONENT_NAME_FORMATTED {
    pub fn setup(mut commands: Commands) -> () {}
    pub fn update() -> () {}
}" > "$COMPONENT_FILE"

echo "Component '$ORIGINAL_NAME' created successfully in 'src/$BASE_FOLDER'."
echo "Files:"
echo " - $MOD_FILE"
echo " - $COMPONENT_FILE"
