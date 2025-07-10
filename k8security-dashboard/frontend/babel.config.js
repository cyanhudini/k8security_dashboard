// babel.config.js
export const presets = [
    // This handles modern JavaScript features (including import/export)
    ['@babel/preset-env', { targets: { node: 'current' } }],

    // This is the CRUCIAL preset that handles JSX
    ['@babel/preset-react', { runtime: 'automatic' }]
];