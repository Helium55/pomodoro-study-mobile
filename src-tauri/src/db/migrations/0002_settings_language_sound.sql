UPDATE settings
SET value = '"ding.wav"'
WHERE key = 'notify.sound_file'
  AND value = '"ding.mp3"';

INSERT OR IGNORE INTO settings(key, value)
VALUES('language', '"zh-CN"');
