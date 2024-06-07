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

export function getParams() {
  return [{
    name: "frequency",
    min: 20,
    max: 20000,
    default: 440,
  }]
}

export function process(input) {
  const frequency = settings.frequency;
  const sampleRate = settings.sample_rate;
  let phase = settings.phase;
  const deltaPhase = frequency / sampleRate;

  // FIXME: input is sometimes an empty array, which causes the following error
  // if (!Array.isArray(input) || input.length === 0) {
  //   console.error('Invalid input array');
  //   return [];
  // }

  const output = input.map((_, index) => {
    phase += deltaPhase;
    if (phase > 1.0) {
      phase -= 1.0;
    }
    const value = Math.sin(phase * 2.0 * Math.PI);
    return value;
  });

  settings.phase = phase;

  return output;
}
