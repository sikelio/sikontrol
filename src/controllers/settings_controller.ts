import { Controller } from '@hotwired/stimulus';
import { invoke } from '@tauri-apps/api';

import type { IPackageJson } from '../interfaces/IPackageJson';

export default class settings_controller extends Controller {
    public static targets: string[] = ['port', 'token', 'version'];

    declare readonly portTarget: HTMLInputElement;
    declare readonly tokenTarget: HTMLInputElement;
    declare readonly versionTarget: HTMLSpanElement;

    public async connect() {
        const packageJson: string = await invoke('get_package_json');

        this.setPackageInfos(JSON.parse(packageJson));
    }

    public saveSettings(e: SubmitEvent) {
        e.preventDefault();

        let errorCount: number = 0;

        if (this.portTarget.value.trim().length === 0) {
            this.showErrorLabel(this.element.querySelector('#port-required'));
            errorCount++;
        } else {
            this.hideErrorLabel(this.element.querySelector('#port-required'));
        }

        if (this.tokenTarget.value.trim().length === 0) {
            this.showErrorLabel(this.element.querySelector('#token-required'));
            errorCount++;
        } else {
            this.hideErrorLabel(this.element.querySelector('#token-required'));
        }

        if (errorCount > 0) {
            return;
        }

        return // TODO: IPC request
    }

    private showErrorLabel(label: HTMLSpanElement | null): void {
        if (label === null) {
            return;
        }

        console.error(label);
        

        label.classList.remove('hidden');
    }

    private hideErrorLabel(label: HTMLSpanElement | null): void {
        if (label === null) {
            return;
        }

        label.classList.add('hidden');
    }

    public toggleAutostart(e: InputEvent) {
        e.preventDefault();

        const checkbox: HTMLInputElement = e.currentTarget as HTMLInputElement;

        return; // TODO: IPC Request
    }

    private setPackageInfos(pkg: IPackageJson) {
        this.versionTarget.textContent = pkg.version
    }
}
