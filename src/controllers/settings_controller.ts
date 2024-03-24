import { Controller } from '@hotwired/stimulus';
import { invoke } from '@tauri-apps/api';
import { enable, isEnabled, disable } from 'tauri-plugin-autostart-api';

import storeManager from '../libs/StoreManager';
import CustomAlert from '../libs/CustomAlert';

import type { IPackageJson } from '../interfaces/IPackageJson';

export default class settings_controller extends Controller {
    public static targets: string[] = ['port', 'token', 'version', 'socketform', 'autostartcheck'];

    declare readonly portTarget: HTMLInputElement;
    declare readonly tokenTarget: HTMLInputElement;
    declare readonly versionTarget: HTMLSpanElement;
    declare readonly socketformTarget: HTMLFormElement;
    declare readonly autostartcheckTarget: HTMLInputElement;

    public async connect() {
        const packageJson: string = await invoke('get_package_json');
        const autostart = await isEnabled();

        if (autostart === true) {
            this.autostartcheckTarget.checked = true;
        }

        this.setPackageInfos(JSON.parse(packageJson));
    }

    public async saveSettings(e: SubmitEvent): Promise<void> {
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

        try {
            await storeManager.setValue('socketConfig', { port: Number(this.portTarget.value), token: this.tokenTarget.value });

            CustomAlert.Toast.fire({
                icon: 'success',
                title: 'Settings saved',
                text: 'Your settings have been saved'
            });

            this.socketformTarget.reset();
        } catch (err: any) {
            CustomAlert.Toast.fire({
                icon: 'error',
                title: 'An error occured',
                text: err
            });
        }
    }

    private showErrorLabel(label: HTMLSpanElement | null): void {
        if (label === null) {
            return;
        }

        label.classList.remove('hidden');
    }

    private hideErrorLabel(label: HTMLSpanElement | null): void {
        if (label === null) {
            return;
        }

        label.classList.add('hidden');
    }

    public async toggleAutostart(e: InputEvent) {
        e.preventDefault();

        const checkbox: HTMLInputElement = e.currentTarget as HTMLInputElement;
        checkbox.disabled = true;

        try {
            if (checkbox.checked === true) {
                await enable();
            } else {
                await disable();
            }
        } catch (err: any) {
            CustomAlert.Toast.fire({
                icon: 'error',
                title: 'An error occured',
                text: err
            });
        }

        checkbox.disabled = false;
    }

    private setPackageInfos(pkg: IPackageJson) {
        this.versionTarget.textContent = pkg.version
    }
}
