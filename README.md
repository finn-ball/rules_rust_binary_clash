# rules_rust_binary_clash

This repository demonstrates a breaking change.

```
bazel run //:app
```

Results in the error:
```
  = note: ld.lld: error: undefined symbol: app_function
          >>> referenced by app.1555d9328bfdc04c-cgu.0
          >>>               bazel-out/k8-fastbuild/bin/app.app.1555d9328bfdc04c-cgu.0.rcgu.o:(app::main::h288a2b8d569cac4d)
          collect2: error: ld returned 1 exit status
```

This can be fixed in one of two ways.

Edit the code:
``` rust
    cc::Build::new()
        .file(c_file)
        // .compile("collision_lib");
        .compile("collision_lib_myapp"); // Flip this to make it work
```

Or edit the commit:
``` python
git_override(
    module_name = "rules_rust",
    # commit = "e413691b6d06f4c0c0df250410a3e93e76d7e200",   # Breaking commit
    commit = "8798aca336b58bfe6ce70265e05fa09257a7b86a",     # Working commit (commit before the above)
    remote = "https://github.com/bazelbuild/rules_rust.git",
)
```
