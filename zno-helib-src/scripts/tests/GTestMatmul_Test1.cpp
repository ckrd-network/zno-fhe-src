TYPED_TEST(GTestMatmul, multipliesWithoutErrors)
{
  HELIB_NTIMER_START(EncodeMartix_MatMul);
  // Templated class so explicit 'this' necessary
  const typename TypeParam::MatrixType& mat = *(this->matrixPtr);
  typename TypeParam::MatrixType::ExecType mat_exec(mat, (this->minimal));
  mat_exec.upgrade();
  HELIB_NTIMER_STOP(EncodeMartix_MatMul);

  // choose a random plaintext vector and encrypt it
  helib::PlaintextArray v(this->ea);
  random(this->ea, v);

  // encrypt the random vector
  helib::Ctxt ctxt(this->secretKey);
  this->ea.encrypt(ctxt, this->secretKey, v);
  helib::Ctxt ctxt2 = ctxt;

  mat_exec.mul(ctxt);

  mul(v, mat); // multiply the plaintext vector

  helib::PlaintextArray v1(this->ea);
  this->ea.decrypt(ctxt, this->secretKey, v1); // decrypt the ciphertext vector

  EXPECT_TRUE(equals(this->ea, v, v1)); // check that we've got the right answer
}

} // namespace
