import { Controller } from '@hotwired/stimulus';
import { invoke } from '@tauri-apps/api';
import { Event as TauriEvent, listen } from '@tauri-apps/api/event';

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

    private canStart: boolean = false;
    private clientList: string[] = [];

    public async connect(): Promise<void> {
        const ip: string | null = await invoke('get_ip');

        this.ipTarget.textContent = ip === null ? 'Not connected' : ip;
        this.canStart = !(ip === null);
        this.fillSocketConfig();

        document.addEventListener('settings-saved', async (): Promise<void> => await this.fillSocketConfig());

        listen('new_client', (e: TauriEvent<string>) => this.newClient(e));        
        listen('client_leave', (e: TauriEvent<string>) => this.clientLeave(e));
        listen('socket_stopped', (e: TauriEvent<unknown>) => this.socketClosed());
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
            const socketConfig: ISocketConfig = await storeManager.getValue('socketConfig') as ISocketConfig;

            if (socketConfig === undefined || socketConfig === null) {
                CustomAlert.Toast.fire({
                    icon: 'warning',
                    title: 'Settings error',
                    text: 'You must set some settings, at least a port.'
                });

                return;
            }

            if (this.canStart === false) {
                CustomAlert.Toast.fire({
                    icon: 'error',
                    title: 'Connection error',
                    text: 'You don\'t have an IP address'
                });

                return;
            }

            this.startbtnTarget.disabled = true;
            this.stopbtnTarget.disabled = false;
            this.servivestatusTarget.innerText = 'Started';

            await invoke('start_server', { port: socketConfig.port });

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

    private newClient(event: TauriEvent<string>): void {
        if (this.clientList.includes(event.payload) === false) {
            const clientSpan: HTMLSpanElement = document.createElement('span');
            clientSpan.innerText = `- ${event.payload}`;

            const clientLine: HTMLLIElement = document.createElement('li');
            clientLine.appendChild(clientSpan);
            clientLine.id = event.payload;

            this.devicesTarget.appendChild(clientLine);
            this.clientList.push(event.payload);
        }
    }

    private clientLeave(event: TauriEvent<string>): void {
        const clientLine = this.element.querySelector(`#${event.payload}`);

        console.log(clientLine);

        if (clientLine == null) {
            return;
        }

        clientLine.remove();
        
        const clientIndex: number = this.clientList.indexOf(event.payload);
        this.clientList.splice(clientIndex, 1);
    }

    private socketClosed(): void {
        this.devicesTarget.innerHTML = '';
    }
}
