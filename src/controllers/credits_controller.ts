import { Controller } from '@hotwired/stimulus';
import { invoke } from '@tauri-apps/api';
import toml from 'toml';

import type { IPackageJson } from '../interfaces/IPackageJson';
import type { IToml, ITomlDependency } from '../interfaces/IToml';

export default class credits_controller extends Controller {
    public static targets: string[] = ['npmlibs', 'npmdevlibs', 'cargolibs'];

    declare readonly npmlibsTarget: HTMLUListElement;
    declare readonly npmdevlibsTarget: HTMLUListElement;
    declare readonly cargolibsTarget: HTMLUListElement;

    private readonly npmBaseLink: string = 'https://www.npmjs.com/package/';
    private readonly crateBaseLink: string = 'https://crates.io/crates/';

    public async connect() {
        const packgeJson: string = await invoke('get_package_json');
        const cargoToml: string = await invoke('get_package_rust');

        this.insertNpmLibs(JSON.parse(packgeJson));
        this.insertCargoLibs(toml.parse(cargoToml));
    }

    private insertNpmLibs(pkg: IPackageJson) {
        Object.keys(pkg.dependencies).forEach((dependency: string) => {
            const libLink: HTMLAnchorElement = document.createElement('a');
            libLink.href = `${this.npmBaseLink}${dependency}`;
            libLink.target = '_blank';
            libLink.classList.add('hover:text-gray-500', 'hover:underline');
            libLink.textContent = `- ${dependency} - ${pkg.dependencies[dependency].replace('^', 'v')}`;;

            const libItem: HTMLLIElement = document.createElement('li');
            libItem.appendChild(libLink);

            this.npmlibsTarget.appendChild(libItem);
        });

        Object.keys(pkg.devDependencies).forEach((devDependency: string) => {
            const libLink: HTMLAnchorElement = document.createElement('a');
            libLink.href = `${this.npmBaseLink}${devDependency}`;
            libLink.target = '_blank';
            libLink.classList.add('hover:text-gray-500', 'hover:underline');
            libLink.textContent = `- ${devDependency} - ${pkg.devDependencies[devDependency].replace('^', 'v')}`;;

            const libItem: HTMLLIElement = document.createElement('li');
            libItem.appendChild(libLink);

            this.npmdevlibsTarget.appendChild(libItem);
        });
    }

    private insertCargoLibs(pkg: IToml) {
        console.log(pkg);

        Object.keys(pkg.dependencies).forEach((dependency: string) => {
            const libLink: HTMLAnchorElement = document.createElement('a');
            libLink.href = `${this.crateBaseLink}${dependency}`;
            libLink.target = '_blank';
            libLink.classList.add('hover:text-gray-500', 'hover:underline');
            libLink.textContent = `- ${dependency} - v${this.getCargoLibVersion(pkg.dependencies[dependency])}`;

            const libItem: HTMLLIElement = document.createElement('li');
            libItem.appendChild(libLink);

            this.cargolibsTarget.appendChild(libItem);
        });
    }

    private getCargoLibVersion(dependency: string | ITomlDependency): string {
        if (typeof dependency === 'string') {
            return dependency;
        }
        
        return dependency.version;
    }
}