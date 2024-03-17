import { Application } from '@hotwired/stimulus';

import main_controller from './controllers/main_controller';

window.addEventListener('DOMContentLoaded', async (): Promise<void> => {
  const application: Application = Application.start();
  application.register('main', main_controller);
});
