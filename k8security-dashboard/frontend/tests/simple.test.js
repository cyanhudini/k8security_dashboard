import { describe, it, expect } from 'vitest';

describe('Korrektheit der Addition', () => {
  it('should pass a simple test', () => {
    const result = 1 + 1;
    expect(result).toBe(2);
  });

  it('Dies wird fehlschlagen', () => {
    const result = 1 + 1;
    expect(result).toBe(3); 
  });
}
);  