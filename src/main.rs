use cairo_vm::{
    cairo_run::CairoRunConfig,
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
    types::program::Program,
    vm::{
        errors::vm_exception::VmException, runners::cairo_runner::CairoRunner,
        vm_core::VirtualMachine,
    },
};

fn main() {
    exec();
}

fn exec() {
    const PROGRAM_JSON: &str = include_str!("./fib0.json");
    let program = Program::from_bytes(PROGRAM_JSON.as_bytes(), Some("main")).unwrap();
    let cairo_run_config = CairoRunConfig::default();
    let mut hint_executor = BuiltinHintProcessor::new_empty();
    let mut cairo_runner = CairoRunner::new(
        &program,
        cairo_run_config.layout,
        cairo_run_config.proof_mode,
    )
    .unwrap();
    let mut vm = VirtualMachine::new(cairo_run_config.trace_enabled);
    let end = cairo_runner.initialize(&mut vm).unwrap();
    cairo_runner
        .run_until_pc(end, &mut vm, &mut hint_executor)
        .map_err(|err| VmException::from_vm_error(&cairo_runner, &vm, err))
        .unwrap();
    cairo_runner
        .end_run(false, false, &mut vm, &mut hint_executor)
        .unwrap();

    vm.verify_auto_deductions().unwrap();
    cairo_runner.read_return_values(&mut vm).unwrap();
    cairo_runner.relocate(&mut vm).unwrap();
}
