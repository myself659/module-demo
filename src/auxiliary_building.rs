// The generate_energy function, referenced in main.rs
// is defined within the file auxiliary.rs (as we 
// see at in the next file). For each such file we add
// a mod <file> expression.
//
// What does pub do? Pub does make the module visible
// to whoever uses this module (which is our crate,
// main.rs). If mod <file> is not preceeded by pub
// it is only visible within the module.
pub mod auxiliary;

// We add the sub module plug. The module system
// will reference plug.rs.
pub mod plug;
