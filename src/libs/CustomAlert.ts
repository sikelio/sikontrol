import Swal from 'sweetalert2';

export default class CustomAlert {
    public static SA2: typeof Swal = Swal;

    public static Toast: typeof CustomAlert.SA2 = CustomAlert.SA2.mixin({
		toast: true,
		position: 'bottom-end',
		showConfirmButton: false,
		timer: 3000,
		timerProgressBar: true,
		didOpen: (toast: HTMLElement): void => {
			toast.onmouseenter = CustomAlert.SA2.stopTimer;
			toast.onmouseleave = CustomAlert.SA2.resumeTimer;
		}
	});
}
