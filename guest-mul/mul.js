let settings = {
    factor: 1
};

export function set(key, value) {
  settings[key] = value;
}

export function process(input) {

  const multiplier = settings['factor'] || 1;

  return input.map(value => value * multiplier);
}
