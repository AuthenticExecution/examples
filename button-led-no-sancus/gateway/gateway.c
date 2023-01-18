SM_OUTPUT(toggle_led);
SM_OUTPUT(increment_presses);

SM_INPUT(button_pressed, data, data_len) {
    DMSG("Remote button has been pressed\n");

    // temporary: get around a bug
    unsigned char fake_data[2] = {1,2};

    // toggle LED
    toggle_led(NULL, 0);

    // increment occurrences on db
    increment_presses(fake_data, 2);
}