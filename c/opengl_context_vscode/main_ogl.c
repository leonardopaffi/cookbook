//Make it so we don't need to include any other C files in our build.
#define CNFG_IMPLEMENTATION

// Gives an OpenGL context
#define CNFGOGL

#include "rawdraw_sf.h"

void HandleKey( int keycode, int bDown ) { }
void HandleButton( int x, int y, int button, int bDown ) { }
void HandleMotion( int x, int y, int mask ) { }
void HandleDestroy() { }

struct VR_IVRSystem_FnTable* oSystem;
struct VR_IVROverlay_FnTable* oOverlay;

int main()
{
	CNFGSetup("Example App", 256, 256 );

	while(1)
	{
		CNFGBGColor = 0x000080ff; // Dark Blue Background

		short w, h;
		CNFGClearFrame();
		CNFGHandleInput();

		// Change color to white.
		CNFGColor( 0xffffffff ); 

		CNFGPenX = 1; CNFGPenY = 1;
		CNFGDrawText( "Hello, World", 2 );

		// Display the image and wait for time to display next frame.
		CNFGSwapBuffers();		
	}
}