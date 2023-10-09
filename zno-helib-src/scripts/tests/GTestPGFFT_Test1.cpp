TEST(GTestPGFFT, PGFFTWorksInRange1to100Points)
{
  SetSeed();

  for (long n = 1; n <= 100; n++)
    TestIt(n);
}

