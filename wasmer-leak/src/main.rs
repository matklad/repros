#[derive(wasmer::WasmerEnv, Clone)]
struct Env {
    i: i32,
}

fn main() {
    let n_imports = 128;

    let wat = {
        let imports = (0..n_imports)
            .map(|i| format!(r#"(import "env" "{}" (func (result i32)))"#, i))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            r#"(module
                {}
                (func (export "main")
                    (call 0)
                    drop)
            )"#,
            imports
        )
    };

    let jit = wasmer::Universal::new(wasmer::Singlepass::new());
    let store = wasmer::Store::new(&jit.engine());
    let module = wasmer::Module::new(&store, &wat).unwrap();

    for i in (0..).cycle() {
        // The leak boils down to a hashmap keyed by addresses of things, so we
        // use this ballast to fudge the allocator to make sure that `alloc`
        // doesn't return duplicate addresses.
        let mut ballast = Vec::new();
        for _ in 0..fastrand::usize(..1024) {
            ballast.push(vec![0u8; fastrand::usize(..1024)]);
        }

        let mut io = wasmer::ImportObject::new();
        let mut exports = wasmer::Exports::new();
        for j in 0..n_imports {
            let env = Env { i };
            exports.insert(
                j.to_string(),
                wasmer::Function::new_native_with_env(&store, env, move |e: &Env| e.i),
            );
        }
        io.register("env", exports);
        let instance = wasmer::Instance::new(&module, &io).unwrap();

        instance
            .exports
            .get_function("main")
            .unwrap()
            .call(&[])
            .unwrap();
    }
}
