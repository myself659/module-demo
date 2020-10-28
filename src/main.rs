// bringing the module auxiliary_building into scope
mod auxiliary_building;
mod tower_building;
// instead of writing full paths, we use the name "aux"
// henceforth in the file
use auxiliary_building::auxiliary as aux;
use tower_building::tower as tower;

fn main() {
    println!("Startup of Tower");

    // a function within the auxiliary_building module
    aux::generate_energy();
    tower::start_consumption();
    // new functions from sub module plug
    auxiliary_building::plug::device::do_it();
}
