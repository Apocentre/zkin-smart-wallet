export const g1Uncompressed = (curve, p1Raw) => {
  const p1 = curve.G1.fromObject(p1Raw);
  const buff = new Uint8Array(64); // 64 bytes for G1 uncompressed
  curve.G1.toRprUncompressed(buff, 0, p1);

  return Buffer.from(buff);
}

export const g2Uncompressed = (curve, p2Raw) => {
  const p2 = curve.G2.fromObject(p2Raw);
  const buff = new Uint8Array(128); // 128 bytes for G2 uncompressed
  curve.G2.toRprUncompressed(buff, 0, p2);

  return Buffer.from(buff);
}

export const toHex64Padded = val => BigInt(val).toString(16).padStart(64, "0");
export const to32ByteBuffer = val => Buffer.from(toHex64Padded(val), "hex");
