//! This is the main file
use module_task::modules::task_scheduler::task_scheduler;
#[allow(unused_imports)]
use module_task::modules::{
    employee_hashmap::employee_hashmap_task, employee_task::employee_task, practice::practice,
    student::student_task, student_hashmap::student_hashmap_task, thread_task::thread_task,
};

/// This function is the entry point of the program.
fn main() {
    // practice();

    // employee_hashmap_task();
    // student_task();
    // student_hashmap_task();
    // employee_task();
    // table_tasks();
    // table_hmap();
    // thread_task();
    task_scheduler();
}
