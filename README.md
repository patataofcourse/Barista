# Barista
A launcher for Rhythm Heaven Megamix mods.

## Credits
- Coding by patataofcourse
- Graphics by MilaDraws / MilaDoesStuff
- Music by ThePurpleAnon and TheEggo55
- Program icon by Kievit, DaBluePipe and patataofcourse

### External code adapted for this program
- [Luma3DS plugin loader interaction code](https://github.com/Nanquitas/Luma3DS-Plugin-sample/blob/master/sources/plgldr.c) by Nanquitas
- BCSTM code adapted from NPI-D7's [BCSTM-Player](https://github.com/NPI-D7/BCSTM-Player)


## Building
In order to build (as of devkitARM-crtls v1.2.33), you'll need to manually edit `$DEVKITARM/arm-none-eabi/lib/3dsx.ld` as follows:

```diff
@@ -79,7 +79,7 @@
         . = ALIGN(4);
     } : data
 
-    .tdata : ALIGN(4)
+    .tdata : ALIGN(8)
     {
         __tdata_lma = .;
         *(.tdata)
```

If this fix causes any errors, please open an issue.