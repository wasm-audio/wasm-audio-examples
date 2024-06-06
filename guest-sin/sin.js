const settings = {
  phase: 0.0,
  frequency: 440.0,
  sample_rate: 48000.0,
};

export function set(key, value) {
  switch (key) {
    case 'phase':
      settings.phase = value;
      break;
    case 'frequency':
    case 'freq':
      settings.frequency = value;
      break;
    case 'sample_rate':
    case 'sr':
      settings.sample_rate = value;
      break;
    default:
      console.log(`Unknown parameter: ${key}`);
  }
  console.log(`Set ${key} to ${value}`);
}

export function process(input) {
  const frequency = settings.frequency;
  const sampleRate = settings.sample_rate;
  let phase = settings.phase;
  const deltaPhase = frequency / sampleRate;

  console.log(`Processing with frequency: ${frequency}, sample rate: ${sampleRate}, initial phase: ${phase}`);

  const output = input.map((_, index) => {
    phase += deltaPhase;
    if (phase > 1.0) {
      phase -= 1.0;
    }
    const value = Math.sin(phase * 2.0 * Math.PI);
    if (index < 10) { // 仅打印前10个值
      console.log(`output[${index}] = ${value}`);
    }
    return value;
  });

  settings.phase = phase;

  console.log(`Final phase: ${phase}`);

  return output;
}
