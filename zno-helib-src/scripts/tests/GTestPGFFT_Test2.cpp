TEST(GTestPGFFT, PGFFTWorksInRange256to32768PowerOfTwoPoints)
{
  SetSeed();

  for (long n = 256; n <= 32 * 1024; n *= 2)
    TestIt(n);
}

