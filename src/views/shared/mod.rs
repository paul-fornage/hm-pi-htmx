pub mod register_view;
pub mod register_edit_modal;
pub mod modbus_helpers;
pub mod analog_register;
pub mod boolean_register;
pub mod result_feedback;
pub mod analog_dword_register;

pub use register_view::{EditableAnalogRegister, EditableBooleanRegister};
pub use register_edit_modal::{
    AnalogEditModalTemplate, BooleanEditModalTemplate, WriteErrorModalTemplate,
};
pub use modbus_helpers::{mb_read_bool_helper, mb_read_word_helper};
