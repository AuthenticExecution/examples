SM_OUTPUT(toggle_led);
SM_OUTPUT(increment_presses);

SM_INPUT(button_pressed, data, data_len) {
    DMSG("Remote button has been pressed\n");

    // toggle LED
    OUTPUT(toggle_led, NULL, 0);

    // increment occurrences on db
    OUTPUT(increment_presses, NULL, 0);
}