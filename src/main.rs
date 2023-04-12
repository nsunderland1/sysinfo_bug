use std::ops::Deref;

use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

fn main() {
    // The user passes a UID from the command line
    let my_new_uid: u32 = std::env::args().nth(1).unwrap().parse().unwrap();
    // It shouldn't match the current process's effective UID
    assert_ne!(nix::unistd::Uid::effective().as_raw(), my_new_uid);
    // Now use seteuid to change our effective UID to it
    nix::unistd::seteuid(nix::unistd::Uid::from_raw(my_new_uid)).unwrap();
    // Sanity check that it worked
    assert_eq!(nix::unistd::Uid::effective().as_raw(), my_new_uid);

    // Now check the effective UID using sysinfo
    let system = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new().with_user()),
    );

    let my_pid = sysinfo::get_current_pid().unwrap();
    let my_uid_but_wrong = system.process(my_pid).unwrap().user_id().unwrap();

    // This assertion will fail, as sysinfo will report a UID of 0 (root)
    assert_eq!(*my_uid_but_wrong.deref(), my_new_uid);
}
