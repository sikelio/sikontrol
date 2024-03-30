import { Controller } from '@hotwired/stimulus';
import { invoke } from '@tauri-apps/api';

import CustomAlert from '../libs/CustomAlert';
import storeManager from '../libs/StoreManager';

import type { ISocketConfig } from '../interfaces/ISocket';

export default class app_controller extends Controller<HTMLDivElement> {
    public static targets: string[] = ['ip', 'devices', 'port', 'token', 'startbtn', 'stopbtn', 'servivestatus'];

    declare readonly ipTarget: HTMLSpanElement;
    declare readonly devicesTarget: HTMLUListElement;
    declare readonly portTarget: HTMLSpanElement;
    declare readonly tokenTarget: HTMLSpanElement;
    declare readonly startbtnTarget: HTMLButtonElement;
    declare readonly stopbtnTarget: HTMLButtonElement;
    declare readonly servivestatusTarget: HTMLSpanElement;

    public async connect(): Promise<void> {
        const ip: string = await invoke('get_ip');

        this.ipTarget.textContent = ip;
        this.fillSocketConfig();

        document.addEventListener('settings-saved', async (): Promise<void> => await this.fillSocketConfig());
    }

    private async fillSocketConfig(): Promise<void> {
        const socketConfig = await storeManager.getValue('socketConfig');

        if (socketConfig === undefined || socketConfig === null) {
            this.portTarget.textContent = 'Not set';
            this.tokenTarget.textContent = 'Not set';
        } else {
            this.portTarget.textContent = (socketConfig as ISocketConfig).port.toString();
            this.tokenTarget.textContent = (socketConfig as ISocketConfig).token;
        }

        const isStarted: boolean = await invoke('is_socket_started');

        if (isStarted === true) {
            this.startbtnTarget.disabled = true;
            this.stopbtnTarget.disabled = false;
        } else {
            this.startbtnTarget.disabled = false;
            this.stopbtnTarget.disabled = true;
        }
    }

    public async startServer(e: MouseEvent): Promise<void> {
        e.preventDefault();

        try {
            const socketConfig = await storeManager.getValue('socketConfig');

            if (socketConfig === undefined || socketConfig === null) {
                CustomAlert.Toast.fire({
                    icon: 'warning',
                    title: 'Settings error',
                    text: 'You must set some settings, at least a port.'
                });

                return;
            }

            this.startbtnTarget.disabled = true;
            this.stopbtnTarget.disabled = false;
            this.servivestatusTarget.innerText = 'Started';

            await invoke('start_server', { port: (socketConfig as ISocketConfig).port });

            return;
        } catch (err: any) {
            this.startbtnTarget.disabled = false;
            this.stopbtnTarget.disabled = true;
            this.servivestatusTarget.innerText = 'Stopped';

            return;
        }
    }

    public async stopServer(e: MouseEvent): Promise<void> {
        e.preventDefault();

        try {
            this.startbtnTarget.disabled = false;
            this.stopbtnTarget.disabled = true;
            this.servivestatusTarget.innerText = 'Stopped';

            await invoke('stop_server');
        } catch (err: any) {
            this.startbtnTarget.disabled = true;
            this.stopbtnTarget.disabled = false;
            this.servivestatusTarget.innerText = 'Started';
        }
    }
}
