import { Controller } from '@hotwired/stimulus';
import { invoke } from '@tauri-apps/api';
import toml from 'toml';

import helpersList from '../data/helpers.json';

import type { IPackageJson } from '../interfaces/IPackageJson';
import type { IToml, ITomlDependencyCrates, ITomlDependencyGitHub } from '../interfaces/IToml';
import type { IHelper } from '../interfaces/IHelper';

export default class credits_controller extends Controller {
    public static targets: string[] = ['npmlibs', 'npmdevlibs', 'cargolibs', 'helpers'];

    declare readonly npmlibsTarget: HTMLUListElement;
    declare readonly npmdevlibsTarget: HTMLUListElement;
    declare readonly cargolibsTarget: HTMLUListElement;
    declare readonly helpersTarget: HTMLUListElement;

    private readonly npmBaseLink: string = 'https://www.npmjs.com/package/';
    private readonly crateBaseLink: string = 'https://crates.io/crates/';

    public async connect() {
        const packgeJson: string = await invoke('get_package_json');
        const cargoToml: string = await invoke('get_package_rust');
        const helpers: IHelper[] = helpersList;

        this.insertNpmLibs(JSON.parse(packgeJson));
        this.insertCargoLibs(toml.parse(cargoToml));
        this.insertHelpers(helpers);
    }

    private insertNpmLibs(pkg: IPackageJson) {
        Object.keys(pkg.dependencies).forEach((dependency: string) => {
            const libLink: HTMLAnchorElement = document.createElement('a');
            libLink.href = `${this.npmBaseLink}${dependency}`;
            libLink.target = '_blank';
            libLink.classList.add('hover:text-gray-500', 'hover:underline');
            libLink.textContent = `- ${dependency} - ${this.formatNpmPackageVersion(pkg.dependencies[dependency])}`;

            const libItem: HTMLLIElement = document.createElement('li');
            libItem.appendChild(libLink);

            this.npmlibsTarget.appendChild(libItem);
        });

        Object.keys(pkg.devDependencies).forEach((devDependency: string) => {
            const libLink: HTMLAnchorElement = document.createElement('a');
            libLink.href = `${this.npmBaseLink}${devDependency}`;
            libLink.target = '_blank';
            libLink.classList.add('hover:text-gray-500', 'hover:underline');
            libLink.textContent = `- ${devDependency} - ${this.formatNpmPackageVersion(pkg.devDependencies[devDependency])}`;;

            const libItem: HTMLLIElement = document.createElement('li');
            libItem.appendChild(libLink);

            this.npmdevlibsTarget.appendChild(libItem);
        });
    }

    private insertCargoLibs(pkg: IToml) {
        Object.keys(pkg.dependencies).forEach((dependency: string) => {
            const libLink: HTMLAnchorElement = document.createElement('a');
            libLink.href = `${this.crateBaseLink}${dependency}`;
            libLink.target = '_blank';
            libLink.classList.add('hover:text-gray-500', 'hover:underline');
            libLink.textContent = `- ${dependency} - ${this.getCargoLibVersion(pkg.dependencies[dependency])}`;

            const libItem: HTMLLIElement = document.createElement('li');
            libItem.appendChild(libLink);

            this.cargolibsTarget.appendChild(libItem);
        });
    }

    private insertHelpers(helpers: IHelper[]) {
        helpers.forEach((helper: IHelper) => {
            const helperSpan: HTMLSpanElement = document.createElement('span');
            helperSpan.textContent = helper.username;
            helperSpan.classList.add('text-white', 'mt-2');

            const avatarImg: HTMLImageElement = document.createElement('img');
            avatarImg.classList.add('inline-block', 'h-12', 'w-12', 'rounded-full', 'ring-2', 'ring-white', 'm-auto');
            avatarImg.alt = helper.username;
            avatarImg.src = new URL(`../assets/helpers/${helper.avatar}`, import.meta.url).href;

            const helperLink: HTMLAnchorElement = document.createElement('a');
            helperLink.href = helper.url;
            helperLink.target = '_blank';
            helperLink.classList.add('flex', 'flex-col', 'text-center', 'hover:underline');
            helperLink.appendChild(avatarImg);
            helperLink.appendChild(helperSpan);

            this.helpersTarget.appendChild(helperLink);
        });
    }

    private getCargoLibVersion(dependency: string | ITomlDependencyCrates | ITomlDependencyGitHub): string {
        if (typeof dependency === 'string') {
            return `v${dependency}`;
        } else if ('version' in dependency && 'features' in dependency) {
            return `v${dependency.version}`;
        } else if ('git' in dependency && 'branch' in dependency) {
            return dependency.branch;
        }

        return 'undefined';
    }

    private formatNpmPackageVersion(value: string): string {
        const regex: RegExp = /https:\/\/github\.com\/[a-zA-Z0-9-_]+\/[a-zA-Z0-9-_]+(#\S+)?$/;

        if (regex.test(value) === true) {
            const version = value.match(regex);

            if (version === null) {
                return 'undefined';
            }

            return version[1].replace('#', '');
        }

        return value.replace('^', 'v');
    }
}
