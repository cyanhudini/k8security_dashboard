import { series } from 'gulp';
import { execa } from 'execa';
import fs from 'fs';
import { deleteSync } from 'del';

function isBuildOk() {
  try {
    const stats = fs.statSync('dist');
    return stats.isDirectory();
  } catch {
    return false;
  }
}

export function clean(cb) {
  deleteSync(['dist', 'docs']);
  cb();
}

function build(cb) {
  return execa('npm', ['run', 'build']).then(() => cb()) ;
}

function docu() {
  return execa('npx', ['jsdoc', 'src', '-r', '-d', 'docs'])
    .then(({ stdout }) => console.log(`Erfolgreich: ${stdout}`))
    .catch(({ stderr }) => console.error(`Fehler bei Doku: ${stderr}`));
}

function test() {
  if (isBuildOk()) {
    console.log('Build erfolgreich, führe Tests aus');
    return execa('npm', ['test']);
  } else {
    console.error('Build nicht erfolgreich');
    return Promise.resolve(); 
  }
}

function commitAddAndPush() {
  return execa('git', ['add', '.'])
    .then(() => execa('git', ['commit', '-m', 'Automatischer Build und Push']))
    .then(() => execa('git', ['push']));
}

export default series(clean, build, test, docu, commitAddAndPush);
