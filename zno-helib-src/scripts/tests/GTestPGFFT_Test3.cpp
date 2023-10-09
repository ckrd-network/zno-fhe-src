TEST(GTestPGFFT, PGFFTWorksInRange10000to20000Points)
{
  SetSeed();

  for (long i = 0; i < 100; i++) {
    long n = RandomBnd(10000) + 10000;
    TestIt(n);
  }
}
} // namespace
