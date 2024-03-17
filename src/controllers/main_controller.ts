import { Controller } from '@hotwired/stimulus';

export default class main_controller extends Controller<HTMLBodyElement> {
    public static targets: string[] = ['navbtn', 'page'];

    declare readonly navbtnTargets: HTMLButtonElement[];
    declare readonly pageTargets: HTMLDivElement[];

    public connect(): void {
        console.log('Main controller loaded');
        console.log(this.navbtnTargets);
        console.log(this.pageTargets);
    }

    public changePage(e: MouseEvent): void {
        e.preventDefault();

        const btnIndex: number = this.navbtnTargets.indexOf(e.currentTarget as HTMLButtonElement);

        this.navbtnTargets.forEach((btn: HTMLButtonElement, index: number) => {
            if (index === btnIndex) {
                btn.classList.add('active');
                btn.classList.remove('nonactive');
            } else {
                btn.classList.remove('active');
                btn.classList.add('nonactive');
            }
        });

        
        
        this.showPage(btnIndex);
    }

    public showPage(btnIndex: number): void {
        this.pageTargets.forEach((page: HTMLDivElement, index: number) => {
            page.classList.toggle('hidden', index !== btnIndex);
        });
    }
}
