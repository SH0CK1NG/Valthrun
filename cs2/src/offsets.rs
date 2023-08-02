use anyhow::Context;
use kinterface::ByteSequencePattern;
use obfstr::obfstr;

use crate::{CS2Handle, Module};

/// Offsets which needs to be scaned for on runtime.
/// Mostly global variables.
pub struct CS2Offsets {
    /// Client offset for the local player controller ptr
    pub local_controller: u64,

    /// Client offset for the global entity list ptr
    pub global_entity_list: u64,

    /// Client offset for the global world to screen view matrix
    pub view_matrix: u64,
}

impl CS2Offsets {
    pub fn resolve_offsets(cs2: &CS2Handle) -> anyhow::Result<Self> {
        Ok(Self {
            local_controller: Self::find_local_player_controller_ptr(cs2)?,
            global_entity_list: Self::find_entity_list(cs2)?,
            view_matrix: Self::find_view_matrix(cs2)?
        })
    }

    fn find_local_player_controller_ptr(cs2: &CS2Handle) -> anyhow::Result<u64> {
        // 48 83 3D ? ? ? ? ? 0F 95 -> IsLocalPlayerControllerValid
        let pattern = ByteSequencePattern::parse(obfstr!("48 83 3D ? ? ? ? ? 0F 95")).unwrap();
        let inst_address = cs2
            .find_pattern(Module::Client, &pattern)?
            .with_context(|| obfstr!("failed to find local player controller ptr").to_string())?;

        let address =
            inst_address + cs2.read::<i32>(Module::Client, &[inst_address + 0x03])? as u64 + 0x08;
        log::debug!("Local player controller ptr at {:X}", address);
        Ok(address)
    }

    fn find_entity_list(cs2: &CS2Handle) -> anyhow::Result<u64> {
        // 4C 8B 0D ? ? ? ? 48 89 5C 24 ? 8B -> Global entity list
        let pattern_entity_list =
            ByteSequencePattern::parse(obfstr!("4C 8B 0D ? ? ? ? 48 89 5C 24 ? 8B")).unwrap();
        let inst_address = cs2
            .find_pattern(Module::Client, &pattern_entity_list)?
            .with_context(|| obfstr!("failed to find global entity list pattern").to_string())?;
        let entity_list_address =
            inst_address + cs2.read::<i32>(Module::Client, &[inst_address + 0x03])? as u64 + 0x07;
        log::debug!("Entity list at {:X}", entity_list_address);
        Ok(entity_list_address)
    }

    fn find_view_matrix(cs2: &CS2Handle) -> anyhow::Result<u64> {
        let pattern_entity_list =
            ByteSequencePattern::parse(obfstr!("48 8D 0D ? ? ? ? 48 C1 E0 06")).unwrap();

        let inst_address = cs2
            .find_pattern(Module::Client, &pattern_entity_list)?
            .with_context(|| obfstr!("failed to find view matrix pattern").to_string())?;

        let address =
            inst_address + cs2.read::<i32>(Module::Client, &[inst_address + 0x03])? as u64 + 0x07;
        log::debug!("View Matrix {:X}", address);
        Ok(address)
    }
}