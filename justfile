default: deploy

@restart:
  cargo shuttle project restart --idle-minutes 0
  just deploy

@stop:
  cargo shuttle project stop

@start:
  cargo shuttle project start
  just deploy

@deploy:
  RUST_LOG=cargo_shuttle cargo shuttle deploy --no-test --allow-dirty
