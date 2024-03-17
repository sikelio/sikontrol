import { Application } from '@hotwired/stimulus';

import main_controller from './controllers/main_controller';
import app_controller from './controllers/app_controller';
import credits_controller from './controllers/credits_controller';
import settings_controller from './controllers/settings_controller';

window.addEventListener('DOMContentLoaded', async (): Promise<void> => {
  const application: Application = Application.start();
  application.register('main', main_controller);
  application.register('app', app_controller);
  application.register('credits', credits_controller);
  application.register('settings', settings_controller);
});
