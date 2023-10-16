fn build_bgv(builder: &mut ffi::ContextBuilderWrapper, params: BGVParams) -> UniquePtr<ffi::Context> {
    ffi::set_m(builder, params.m.value() as i64); // Assuming `.value()` returns the underlying integer
    ffi::set_p(builder, params.p.value() as i64);
    ffi::set_r(builder, params.r.value() as i64);
    ffi::set_c(builder, params.c.value() as i64);
    ffi::set_bits(builder, params.bits.value().parse::<i64>().expect("expected an integer"));

    let gens = params.gens.value().iter().map(|&x| x as i64).collect::<cxx::CxxVector<_>>();
    let ords = params.ords.value().iter().map(|&x| x as i64).collect::<cxx::CxxVector<_>>();

    ffi::set_gens(builder, &gens);
    ffi::set_ords(builder, &ords);

    // If there are any additional parameters (like mvec or bootstrappable),
    // set them here in a similar manner, assuming you have the corresponding
    // functions in your ffi module

    // Call the build function from C++
    ffi::build(builder)
}
