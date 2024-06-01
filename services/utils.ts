export function verify_json(input: Object, associated_keys: string[]): boolean {
  let verified = true;
  Object.keys(input).forEach((key) => {
    if (!associated_keys.includes(key)) {
      verified = false;
    }
  });

  return verified;
}
