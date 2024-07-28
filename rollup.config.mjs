import { nodeResolve } from '@rollup/plugin-node-resolve';
import terser from '@rollup/plugin-terser';
import typescript from '@rollup/plugin-typescript';

export default {
    input: 'main.ts',
    output: [
        // {
        //     file: 'bundle.js',
        //     format: 'iife'
        // },
        {
            file: 'public/bundle.min.js',
            format: 'iife',
            name: 'version',
            plugins: [terser()]
        }],
    plugins: [nodeResolve(), typescript()]

}